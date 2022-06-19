use std::{collections::HashMap, error::Error, fs::{self, File}, io::Write, path::PathBuf};

use clap::{Arg, ArgMatches, Command};
use serde_json::{json, Value};

use crate::{
    config::Config,
    error::CustomError,
    request::{request, response::{parse_response, AnkiResponse, response_types::{ModelNamesAndIds, ModelFieldNames, ModelTemplates, ModelStyling}}},
};

pub const COMMAND_NAME: &str = "model";

pub fn generate_command<'a>() -> Command<'a> {
    Command::new(COMMAND_NAME)
        .about("Gets a model from Anki")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .multiple_values(true)
                .takes_value(true)
                .value_name("NAME")
                .required(true),
        )
}

pub async fn invoke(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let path = fs::canonicalize(PathBuf::from("./")).unwrap();

    // Read config from workspace root
    let mut config = match Config::find_config_root(path.as_path()) {
        Some(path) => Config::read_config(path.as_path())?,
        None => return Err(Box::new(CustomError::new("No workspace found"))),
    };

    // Loop through all given model names
    for name in matches.values_of("name").unwrap() {
        // Define params only including the model name in a variable as this is used between multiple requests
        let name_params = json!({ "modelName": name });

        // Define list of requests actions to loop through and initialize vector to store join handles
        let request_data = [
            ("modelNamesAndIds", None),
            ("modelTemplates", Some(name_params.clone())),
            ("modelFieldNames", Some(name_params.clone())),
            ("modelStyling", Some(name_params)),
        ];
        let mut requests = HashMap::new();

        // Get api version and port number from config
        let api_version = config.data.get_api_version();
        let port = config.data.get_port();

        // Loop through actions and spawn request tasks for each
        for (request_name, params) in request_data {
            let data = json!({
                "action": request_name,
                "apiVersion": api_version,
                "params": params,
            })
            .to_string();

            let request = tokio::spawn(async move {
                request(&data, port).await.map_err(|err| format!("{}", err))
            });

            requests.insert(request_name, request);
        }

        // Wait for id request to finish
        let id_response = requests.remove("modelNamesAndIds").unwrap().await??;
        let ids = parse_response::<ModelNamesAndIds>(id_response);

        // Create closure for aborting remaining requests in case of an error
        let abort = || {
            for request in requests.values() {
                request.abort();
            }
        };

        let ask_for_confirmation = |message: &str| {
            print!("{} [y/N] ", message);
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            input.trim().to_lowercase() == "y"
        };

        // Check if files for model already exists in workspace
        // Asks user if they want to overwrite if present
        match config.data.get_model_mut(name) {
            Ok(model) => {
                // Check if id for model exists in Anki
                if let Some(anki_id) = ids.get(name) {
                    let local_id = model.id.clone();

                    // Check if ids are the same
                    if local_id != *anki_id {
                        println!("Model ID differs between local and remote.");

                        // Check if user wants to overwrite
                        if ask_for_confirmation("Should the local model be overwritten?") {
                            println!("Overwriting local model.");

                            // Overwrite model id
                            model.id = anki_id.clone();
                        } else {
                            println!("Skipping model with name `{}`.", name);
                            abort();
                            continue;
                        }
                    } else {
                        println!("Model with name `{}` already exists.", name);
                        println!("Non-pushed changes will be lost.");

                        // Check if user wants to overwrite
                        if ask_for_confirmation("Should the local model be overwritten?") {
                            println!("Overwriting local model.");
                        } else {
                            println!("Skipping model with name `{}`", name);
                            abort();
                            continue;
                        }
                    }
                } else {
                    println!("Model with name `{}` does not exist in Anki.", name);
                    println!("Use `akpt new model --name [NAME]` to create a new model.");
                    println!("Skipping model with name `{}`", name);
                    abort();
                    continue;
                }
            }
            Err(_) => {
                println!("Creating model with name `{}`", name);

                // Get path to model folder
                let model_path = config.get_path().join("models").join(name);
                let card_path = model_path.join("cards");

                // Create model and cards folder
                fs::create_dir_all(card_path)?;
            }
        }

        // Wait for remaining requests to finish and assure all have succeded
        let field_response = requests.remove("modelFieldNames").unwrap().await??;
        let field_names = parse_response::<ModelFieldNames>(field_response);

        let template_response = requests.remove("modelTemplates").unwrap().await??;
        let templates = parse_response::<ModelTemplates>(template_response);

        let styling_response = requests.remove("modelStyling").unwrap().await??;
        let styling = parse_response::<ModelStyling>(styling_response).css;

        todo!()
    }

    config.write()?;

    Ok(())
}
