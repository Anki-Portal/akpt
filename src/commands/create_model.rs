use clap::{Command, Arg, ArgMatches};

pub fn generate_command<'a>() -> Command<'a> {
    Command::new("create-model")
        .about("Creates a new model")
        .arg(Arg::new("name")
            .short('n')
            .long("name")
            .index(1)
            .multiple_values(true)
            .takes_value(true)
            .value_name("NAME")
            .required(true))
        .arg(Arg::new("cloze")
            .short('c')
            .long("cloze")
            .value_name("CLOZE")
    )   
}

pub fn invoke(matches: &ArgMatches) -> Result<(), String> {
    Ok(())
}