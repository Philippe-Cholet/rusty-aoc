use std::collections::HashMap;

use common::{bail, Context, Error, Part, Part1, Part2, Result};

// Names are unused but I don't want to delete those, it could have been used in part 2.
#[derive(Debug)]
enum LsEntry<'a> {
    Dir {
        #[allow(unused)]
        dirname: &'a str,
    },
    File {
        size: usize,
        #[allow(unused)]
        filename: &'a str,
    },
}

#[derive(Debug)]
enum Command<'a> {
    Cd { path: &'a str },
    Ls { entries: Vec<LsEntry<'a>> },
}

impl<'a> TryFrom<&'a str> for Command<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self> {
        let cmd = s.strip_prefix("$ ").context("No dollar")?;
        Ok(if cmd == "ls" {
            Self::Ls { entries: vec![] }
        } else {
            Self::Cd {
                path: cmd.strip_prefix("cd ").context("Not cd")?,
            }
        })
    }
}

impl<'a> TryFrom<&'a str> for LsEntry<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self> {
        let (t, name) = s.split_once(' ').context("No whitespace")?;
        Ok(if t == "dir" {
            Self::Dir { dirname: name }
        } else {
            let size = t.parse()?;
            Self::File {
                size,
                filename: name,
            }
        })
    }
}

/// No Space Left On Device
#[allow(clippy::expect_used)]
pub fn solver(part: Part, input: &str) -> Result<String> {
    let mut cmds = vec![];
    for line in input.lines() {
        if let Ok(cmd) = line.try_into() {
            cmds.push(cmd);
        } else {
            match cmds
                .last_mut()
                .context("Not a command and no previous command to the output of")?
            {
                Command::Cd { .. } => bail!("The cd command does not have output"),
                Command::Ls { entries } => entries.push(line.try_into()?),
            }
        }
    }
    let mut parts = vec![];
    let mut folder_sizes = HashMap::new();
    for cmd in cmds {
        match cmd {
            Command::Cd { path: "/" } => parts.clear(),
            Command::Cd { path: "." } => {} // not needed
            Command::Cd { path: ".." } => {
                parts.pop().context("Going rogue with the filesystem")?;
            }
            Command::Cd { path } => {
                debug_assert!(!path.contains('/'));
                parts.push(path);
            }
            Command::Ls { entries } => {
                let mut folders = vec!["/".to_owned()];
                for part in &parts {
                    let prev = folders.last().expect("can not be empty");
                    folders.push(format!("{prev}{part}/"));
                }
                for entry in entries {
                    if let LsEntry::File { size, .. } = entry {
                        for folder in &folders {
                            *folder_sizes.entry(folder.clone()).or_insert(0) += size;
                        }
                    }
                }
            }
        }
    }
    let result: usize = match part {
        Part1 => folder_sizes
            .into_values()
            .filter(|size| size <= &100_000)
            .sum(),
        Part2 => {
            let to_free = folder_sizes
                .get("/")
                .context("No root")?
                .checked_sub(40_000_000)
                .context("Nothing to free")?;
            folder_sizes
                .into_values()
                .filter(|size| size >= &to_free)
                .min()
                .context("No solution")?
        }
    };
    Ok(result.to_string())
}

pub const INPUTS: [&str; 2] = [
    "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
",
    include_str!("input.txt"),
];

#[test]
fn solver_22_07() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "95437");
    assert_eq!(solver(Part1, INPUTS[1])?, "1334506");
    assert_eq!(solver(Part2, INPUTS[0])?, "24933642");
    assert_eq!(solver(Part2, INPUTS[1])?, "7421137");
    Ok(())
}
