use std::{
    fs::{File},
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::error::CustomError;

use self::model::ModelConfig;

mod field;
mod model;

pub const CONFIG_FILE: &str = "akptconfig.json";

pub struct Config {
    pub data: ConfigData,
    path: PathBuf,
}
impl Config {
    /// Creates a new instance of a `Config` object.
    /// 
    /// ## Arguments
    /// 
    /// * `path` - The path to the directory where the config file should be stored.
    /// * `data` - The data to store in the config file.
    /// 
    pub fn new(root_dir: &Path, data: ConfigData) -> Self {
        let path = root_dir.join(CONFIG_FILE);

        Config { data, path }
    }

    /// Writes data to the config file.
    pub fn write(&self) -> Result<(), String> {
        let mut file = File::create(&self.path).map_err(|err| err.to_string())?;

        let json = serde_json::to_string_pretty(&self.data).map_err(|err| err.to_string())?;

        file.write_all(json.as_bytes())
            .map_err(|err| err.to_string())?;

        Ok(())
    }

    /// Reads data from the config file.
    /// 
    /// ## Arguments
    /// 
    /// * `path` - The path to the the config file.
    /// 
    pub fn read_config(path: &Path) -> Result<Config, String> {
        let file = File::open(path.join(CONFIG_FILE)).map_err(|err| format!("{}", err))?;

        let data: ConfigData = serde_json::from_reader(file).map_err(|err| format!("{}", err))?;

        Ok(Config { data, path: path.to_path_buf() })
    }

    /// Searches all parent directories for a config file.
    /// 
    /// ## Arguments
    /// 
    /// * `path` - The path to the directory to start searching from.
    /// 
    /// ## Returns
    /// 
    /// The path to the directory where the config file is located.
    /// 
    pub fn find_config_root(path: &Path) -> Option<PathBuf> {
        for parent in path.ancestors() {
            let config_path = parent.join(CONFIG_FILE);

            if config_path.exists() {
                return Some(parent.to_path_buf());
            }
        }

        None
    }

    /// Gets the path to the workspace folder
    /// 
    /// ## Returns
    /// 
    /// The path to the workspace folder.
    pub fn get_path(&self) -> &Path {
        &self.path
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConfigData {
    #[serde(rename = "apiVersion")]
    api_version: u32,
    port: u32,
    models: Vec<ModelConfig>,
}

impl ConfigData {
    pub fn new(api_version: u32, port: u32) -> Self {
        Self {
            api_version,
            port,
            models: Vec::new(),
        }
    }
    pub fn add_model(&mut self, model: ModelConfig) -> Result<(), String> {
        if self.models.contains(&model) {
            return Err(format!("Model {} already exists", model.name));
        }

        self.models.push(model);

        Ok(())
    }

    /// Gets the model config for the given model name.
    /// 
    /// ## Arguments
    /// 
    /// * `name` - The name of the model to get the config for.
    /// 
    /// ## Returns
    /// 
    /// Result resolving to the model config for the given model name.
    pub fn get_model(&self, name: &str) -> Result<ModelConfig, String> {
        let model = self.models.iter().find(|model| model.name == name);

        match model {
            Some(model) => Ok(model.clone()),
            None => Err(format!("Model {} not found", name)),
        }
    }

    /// Gets the model config as a mutable reference for the given model ID.
    /// 
    /// ## Arguments
    /// 
    /// * `name` - The name of the model to get the config for.
    /// 
    /// ## Returns
    /// 
    /// Result resolving to a mutable reference to the model config for the given model name.
    pub fn get_model_mut(&mut self, name: &str) -> Result<&mut ModelConfig, CustomError> {
        let model = self.models.iter_mut().find(|model| model.name == name);

        match model {
            Some(model) => Ok(model),
            None => Err(CustomError::new(&format!("Model {} not found", name))),
        }
    }

    /// Gets the api version used in the workspace.
    /// 
    /// ## Returns
    /// 
    /// The api version used in the workspace.
    pub fn get_api_version(&self) -> u32 {
        self.api_version
    }

    /// Gets the port used to connect to AnkiConnect.
    /// 
    /// ## Returns
    /// 
    /// The port used to connect to AnkiConnect.
    pub fn get_port(&self) -> u32 {
        self.port
    }
}
