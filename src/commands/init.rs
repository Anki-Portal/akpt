use std::{env, path::{Path, PathBuf}, fs::File, io::Write};

use clap::{Arg, ArgMatches, Command};

use crate::types::config::Config;

pub fn generate_command<'a>() -> Command<'a> {
    Command::new("init").about("Initialize AKPT project").arg(
        Arg::new("version")
            .help("Specifies version of the AnkiPortal API to target")
            .short('v')
            .long("version")
            .takes_value(true)
            .value_name("VERSION")
            .default_value("6"),
    )
}

pub fn invoke(matches: &ArgMatches, path: &PathBuf) -> Result<(), String> {

    // Check that location is a directory
    if !path.is_dir() {
        return Err(format!("{} is not a directory", path.display()));
    } 

    // Check that location isn't already an AKPT workspace
    if path.join("akptconfig.json").exists() {
        return Err("Directory is already an AKPT workspace".to_string());
    }

    // Get API version from args
    let version = if let Ok(version) = matches.value_of("version").unwrap().parse::<u32>() {
        version
    } else {
        return Err("Version must be a number".to_string());
    };

    // Create config and write to file
    let config = Config::new(version);

    let config_string = serde_json::to_string(&config).unwrap();

    let mut config_file = File::create(path.join("akptconfig.json")).unwrap();
    config_file.write_all(config_string.as_bytes()).unwrap();

    Ok(())
}
