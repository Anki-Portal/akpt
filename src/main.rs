use std::{path::{Path, PathBuf}, fs};

use clap::{Arg, Command};
use commands::*;

mod commands {
    pub mod create_model;
    pub mod get_model;
    pub mod init;
}

mod types {
    pub mod config;
}

#[macro_use]
mod macros;

fn main() {
    let commands = repeat_over_modules!(generate_command in get_model, create_model, init);

    let matches = Command::new("AKPT")
        .about("CLI tool for communication with Anki")
        .author("Ludwig Granstedt <ludwiggranstedt@gmail.com>")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .takes_value(true)
                .value_name("PATH")
                .default_value("./"),
        )
        .subcommands(commands)
        .get_matches();

    let path_value = matches.value_of("PATH").unwrap();
    let mut path = PathBuf::from(path_value);

    if path.is_relative() {
        let old_path = path.clone();
        path = fs::canonicalize(old_path).unwrap();
    }

    let result = match matches.subcommand() {
        Some(("get-model", matches)) => get_model::invoke(matches),
        Some(("create-model", matches)) => create_model::invoke(matches),
        Some(("init", matches)) => init::invoke(matches, &path),
        _ => Ok(()),
    };

    match result {
        Ok(_) => {}
        Err(error) => {
            println!("{}", error);
        }
    }
}
