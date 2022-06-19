use std::{process::exit, error::Error};

use clap::{Command};
use commands::*;


#[macro_use]
mod macros;

mod commands {
    pub mod get;
    pub mod init;
    pub mod new;
}

mod config;
mod error;
mod request;

mod tools {
    pub mod workspace;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let commands = repeat_over_modules!(generate_command in get, init, new);

    let matches = Command::new("AKPT")
        .about("CLI tool for communication with Anki")
        .author("Ludwig Granstedt <ludwiggranstedt@gmail.com>")
        .subcommands(commands)
        .get_matches();

    match matches.subcommand() {
        Some((get::COMMAND_NAME, matches)) => get::invoke(matches).await,
        Some((init::COMMAND_NAME, matches)) => init::invoke(matches),
        Some((new::COMMAND_NAME, matches)) => new::invoke(matches),
        _ => Ok(()),
    }
}
