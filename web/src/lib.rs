use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use clap::{Parser, Subcommand};

use common::{Day, Part, Year};

const WEBSITE: &str = "https://adventofcode.com";

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to open a web page: {0}")]
    OpenWebpage(io::Error),

    #[error("Invalid session cookie: {0}")]
    InvalidSessionCookie(String),

    #[error("Failed to download from the website: {0}")]
    WebsiteDownload(Box<ureq::Error>),

    #[error("More than 10mb of text?! {0}")]
    WebsiteString(io::Error),

    #[error("Failed to write to the disk: {0}")]
    WriteToDisk(io::Error),

    #[error("Failed to submit to the website: {0}")]
    WebsiteSubmit(Box<ureq::Error>),
}

#[derive(Debug, Parser)]
#[command(
    name = "web",
    about = "Simple interactions with adventofcode.com",
    long_about = None,
)]
pub struct Cli {
    year: Year,
    day: Day,

    /// Session cookie: environment variable, filepath or simply the cookie.
    #[arg(short, long, default_value = "AOC_TOKEN")]
    token: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Open webpages in your browser
    #[command(visible_alias = "o")]
    Open {
        /// Open the calendar page
        #[arg(short, long)]
        calendar: bool,

        /// Open the puzzle description
        #[arg(short, long)]
        description: bool,

        /// Open the puzzle input
        #[arg(short, long)]
        input: bool,
    },

    /// Download files to the disk
    #[command(visible_alias = "dl")]
    Download {
        /// Save the calendar page
        #[arg(short, long, value_name = "FILEPATH")]
        calendar: Option<PathBuf>,

        /// Save the puzzle description
        #[arg(short, long, value_name = "FILEPATH")]
        description: Option<PathBuf>,

        /// Save the puzzle input
        #[arg(short, long, value_name = "FILEPATH")]
        input: Option<PathBuf>,
    },

    /// Submit an answer
    #[command(visible_alias = "s")]
    Submit {
        /// Puzzle part
        part: Part,

        /// Puzzle answer
        answer: String,
    },
}

#[derive(Debug)]
struct AocAgent {
    agent: ureq::Agent,
    cookie: String,
}

impl AocAgent {
    fn from_token(token: &str) -> Result<Self> {
        let agent = ureq::AgentBuilder::new()
            .https_only(true)
            .user_agent("github.com/Philippe-Cholet/rusty-aoc")
            .build();
        let token = token.trim();
        if token.is_empty() || !token.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(Error::InvalidSessionCookie(token.to_owned()));
        }
        let cookie = format!("session={token}");
        Ok(Self { agent, cookie })
    }

    fn from_env(env: &str) -> Result<Self> {
        env::var(env)
            .map_err(|_| Error::InvalidSessionCookie(env.to_owned()))
            .and_then(|token| Self::from_token(&token))
    }

    fn from_file<P: AsRef<Path>>(filepath: P) -> Result<Self> {
        std::fs::read_to_string(&filepath)
            .map_err(|_| Error::InvalidSessionCookie(filepath.as_ref().display().to_string()))
            .and_then(|token| Self::from_token(&token))
    }

    fn download_url(&self, url: &str) -> Result<String> {
        // Basic way to be nice with the server. Too basic?!
        std::thread::sleep(Duration::from_secs(2));
        self.agent
            .get(url)
            .set("content-type", "text/plain")
            .set("cookie", &self.cookie)
            .timeout(Duration::from_secs(15))
            .call()
            .map_err(Box::new)
            .map_err(Error::WebsiteDownload)?
            .into_string()
            .map_err(Error::WebsiteString)
    }

    fn submit_answer(&self, url: &str, part: Part, answer: &str) -> Result<String> {
        self.agent
            .post(url)
            .set("content-type", "application/x-www-form-urlencoded")
            .set("cookie", &self.cookie)
            .timeout(Duration::from_secs(15))
            .query("level", part.value("1", "2"))
            .query("answer", answer)
            .call()
            .map_err(Box::new)
            .map_err(Error::WebsiteSubmit)?
            .into_string()
            .map_err(Error::WebsiteString)
    }
}

impl Cli {
    #[must_use]
    pub fn new(year: Year, day: Day, token: Option<String>, command: Commands) -> Self {
        Self {
            year,
            day,
            token: token.unwrap_or_else(|| "AOC_TOKEN".to_owned()),
            command,
        }
    }

    pub fn with_command(&mut self, command: Commands) -> &mut Self {
        self.command = command;
        self
    }

    fn calendar_url(&self) -> String {
        let year: i32 = self.year.into();
        format!("{WEBSITE}/{year}")
    }

    fn description_url(&self) -> String {
        let year: i32 = self.year.into();
        let day: u8 = self.day.into();
        format!("{WEBSITE}/{year}/day/{day}")
    }

    fn input_url(&self) -> String {
        let year: i32 = self.year.into();
        let day: u8 = self.day.into();
        format!("{WEBSITE}/{year}/day/{day}/input")
    }

    fn answer_url(&self) -> String {
        let year: i32 = self.year.into();
        let day: u8 = self.day.into();
        format!("{WEBSITE}/{year}/day/{day}/answer")
    }

    // TODO: Improve!
    fn aoc_agent(&self) -> Result<AocAgent> {
        AocAgent::from_env(&self.token)
            .or_else(|_| AocAgent::from_file(&self.token))
            .or_else(|_| AocAgent::from_token(&self.token))
    }

    pub fn run(&self) -> Result<()> {
        match &self.command {
            &Commands::Open {
                calendar,
                description,
                input,
            } => {
                for (open, url) in [
                    (calendar, self.calendar_url()),
                    (description, self.description_url()),
                    (input, self.input_url()),
                ] {
                    if open {
                        webbrowser::open(&url).map_err(Error::OpenWebpage)?;
                    }
                }
            }
            Commands::Download {
                calendar,
                description,
                input,
            } => {
                let agent = self.aoc_agent()?;
                for (filepath, url) in [
                    (calendar, self.calendar_url()),
                    (description, self.description_url()),
                    (input, self.input_url()),
                ] {
                    if let Some(filepath) = filepath {
                        if filepath.exists() {
                            eprintln!("This file already exists: {}", filepath.display());
                        } else {
                            let text = agent.download_url(&url)?;
                            fs::OpenOptions::new()
                                .create_new(true)
                                .write(true)
                                .open(filepath)
                                .map_err(Error::WriteToDisk)?
                                .write_all(text.as_bytes())
                                .map_err(Error::WriteToDisk)?;
                        }
                    }
                }
            }
            Commands::Submit { part, answer } => {
                let html = self
                    .aoc_agent()?
                    .submit_answer(&self.answer_url(), *part, answer)?;
                println!("{html}");
            }
        }
        Ok(())
    }
}
