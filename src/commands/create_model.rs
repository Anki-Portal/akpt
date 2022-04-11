use clap::{Arg, ArgMatches, Command};

pub const COMMAND_NAME: &str = "create-model";

pub fn generate_command<'a>() -> Command<'a> {
    Command::new(COMMAND_NAME)
        .about("Creates a new model")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .index(1)
                .multiple_values(true)
                .takes_value(true)
                .value_name("NAME")
                .required(true),
        )
        .arg(
            Arg::new("cloze")
                .short('c')
                .long("cloze")
                .value_name("CLOZE"),
        )
}

pub fn invoke(_matches: &ArgMatches) -> Result<(), String> {
    Ok(())
}
