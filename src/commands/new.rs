use std::error::Error;

use clap::{Command, ArgMatches};

mod model;

pub const COMMAND_NAME: &str = "new";

pub fn generate_command<'a>() -> Command<'a> {
    Command::new(COMMAND_NAME)
        .about("A collection of commands that creates things")
        .subcommands([
            model::generate_command()])
}

pub fn invoke(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    match matches.subcommand() {
        Some((model::COMMAND_NAME, matches)) => model::invoke(matches),
        _ => Ok(()),
    }
}