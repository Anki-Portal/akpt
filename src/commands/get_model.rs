use clap::{Arg, Command};

pub fn generate_command<'a>() -> Command<'a> {
    Command::new("get").about("Get model from Anki").arg(
        Arg::new("name")
            .short('n')
            .long("name")
            .index(1)
            .multiple_values(true)
            .takes_value(true)
            .value_name("NAME")
            .required(true),
    )
}

pub fn get_model() {

}