use std::error::Error;

use clap::{ArgMatches, Command};

mod model;

pub const COMMAND_NAME: &str = "get";

pub fn generate_command<'a>() -> Command<'a> {
    Command::new("get")
        .about("A collection of command that fetches something from an external source (i.e ANKI)")
        .subcommands([model::generate_command()])
}

pub async fn invoke(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    match matches.subcommand() {
        Some((model::COMMAND_NAME, matches)) => model::invoke(matches).await,
        _ => Ok(()),
    }
}
