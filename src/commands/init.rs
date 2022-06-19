use std::{error::Error, fs, path::PathBuf};

use clap::{Arg, ArgMatches, Command};

use crate::{
    config::{Config, ConfigData},
    error::CustomError,
};

pub const COMMAND_NAME: &str = "init";

pub fn generate_command<'a>() -> Command<'a> {
    Command::new(COMMAND_NAME)
        .about("Initialize AKPT project")
        .args([
            Arg::new("version")
                .help("Version of the AnkiPortal API to target")
                .short('v')
                .long("version")
                .takes_value(true)
                .value_name("VERSION")
                .default_value("6"),
            Arg::new("port")
                .help("Port to connect with Anki through")
                .short('p')
                .long("port")
                .takes_value(true)
                .value_name("PORT")
                .default_value("8765"),
        ])
}

pub fn invoke(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let path = fs::canonicalize(PathBuf::from("./")).unwrap();

    // Check that location is a directory
    if !path.is_dir() {
        let message = format!("{} is not a directory", path.display());

        return Err(Box::new(CustomError::new(&message)));
    }

    // Check that location isn't already an AKPT workspace
    if let Some(_config) = Config::find_config_root(path.as_path()) {
        let message = format!("{} is already an AKPT workspace", path.display());

        return Err(Box::new(CustomError::new(&message)));
    }

    // Get API version from args
    let version = if let Ok(version) = matches.value_of("version").unwrap().parse::<u32>() {
        version
    } else {
        return Err(Box::new(CustomError::new("Invalid API version")));
    };

    // Get port from args
    let port = if let Ok(port) = matches.value_of("port").unwrap().parse::<u32>() {
        port
    } else {
        return Err(Box::new(CustomError::new("Invalid port")));
    };

    // Create config and write to file
    let config_data = ConfigData::new(version, port);
    let config = Config::new(path.as_path(), config_data);
    config.write()?;

    // Create folders for models and notes
    fs::create_dir(path.join("models")).map_err(|err| format!("{}", err))?;
    fs::create_dir(path.join("notes")).map_err(|err| format!("{}", err))?;

    println!("Successfully initialized AKPT workspace");
    Ok(())
}
