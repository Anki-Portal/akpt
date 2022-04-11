use std::{fs, path::PathBuf};

use clap::{Arg, ArgMatches, Command};

use crate::config::Config;

pub const COMMAND_NAME: &str = "get-model";

pub fn generate_command<'a>() -> Command<'a> {
    Command::new(COMMAND_NAME).about("Get model from Anki").arg(
        Arg::new("name")
            .short('n')
            .long("name")
            .multiple_values(true)
            .takes_value(true)
            .value_name("NAME")
            .required(true),
    )
}

pub fn invoke(_matches: &ArgMatches) -> Result<(), String> {
    let path = fs::canonicalize(PathBuf::from("./")).unwrap();

    let config = match Config::find_config(path.as_path()) {
        Some(path) => Config::read_config(path.as_path())?,
        None => return Err("No workspace found".to_string()),
    };

    

    Ok(())
}
