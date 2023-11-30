use clap::Parser;

use web::{Cli, Result};

fn main() -> Result<()> {
    Cli::parse().run()
}
