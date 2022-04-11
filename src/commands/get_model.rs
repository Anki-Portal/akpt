use clap::{Arg, Command, ArgMatches};

pub fn generate_command<'a>() -> Command<'a> {
    Command::new("get-model").about("Get model from Anki").arg(
        Arg::new("name")
            .short('n')
            .long("name")
            .multiple_values(true)
            .takes_value(true)
            .value_name("NAME")
            .required(true),
    )
}

pub fn invoke(matches: &ArgMatches) -> Result<(), String> {
    Ok(())
}