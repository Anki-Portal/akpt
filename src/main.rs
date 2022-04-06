use clap::{Arg, Command};
use commands::*;

mod commands {
    pub mod get_model;
    pub mod create_model;
}

mod types {
    pub mod error;
    pub mod request;
    pub mod response;
}

#[macro_use]
mod macros;

fn main() {
    let commands = repeat_over_modules!(generate_command for get_model, create_model);    
}
