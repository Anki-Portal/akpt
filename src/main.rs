use clap::{Arg, Command};
use commands::*;

mod commands {
    pub mod create_model;
    pub mod get_model;
}

mod types {
    pub mod error;
    pub mod request;
    pub mod response;
}

#[macro_use]
mod macros;

fn main() {
    let commands = repeat_over_modules!(generate_command in get_model, create_model);

    let matches = Command::new("AKPT")
        .about("CLI tool for communication with Anki")
        .author("Ludwig Granstedt <ludwiggranstedt@gmail.com>")
        .subcommands(commands)
        .get_matches();
}
