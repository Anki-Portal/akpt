use std::{fs, path::PathBuf, process::exit};

use clap::{Arg, Command};
use commands::*;

mod commands {
    pub mod create_model;
    pub mod get_model;
    pub mod init;
}

mod config;

mod tools {
    pub mod workspace;
}

#[macro_use]
mod macros;

fn main() {
    let commands = repeat_over_modules!(generate_command in get_model, create_model, init);

    let matches = Command::new("AKPT")
        .about("CLI tool for communication with Anki")
        .author("Ludwig Granstedt <ludwiggranstedt@gmail.com>")
        .subcommands(commands)
        .get_matches();

    let result = match matches.subcommand() {
        Some((get_model::COMMAND_NAME, matches)) => get_model::invoke(matches),
        Some((create_model::COMMAND_NAME, matches)) => create_model::invoke(matches),
        Some((init::COMMAND_NAME, matches)) => init::invoke(matches),
        _ => Ok(()),
    };

    match result {
        Ok(_) => {}
        Err(error) => {
            println!("{}", error);
            exit(1);
        }
    }
}
