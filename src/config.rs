use std::{
    fs::{File, self},
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

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
        let file = File::open(path).map_err(|err| format!("{}", err))?;

        let data: ConfigData = serde_json::from_reader(file).map_err(|err| format!("{}", err))?;

        Ok(Config { data, path: path.to_path_buf() })
    }

    pub fn find_config(path: &Path) -> Option<PathBuf> {
        for parent in path.ancestors() {
            let config_path = parent.join(CONFIG_FILE);

            if config_path.exists() {
                return Some(config_path);
            }
        }

        None
    }

    pub fn find_config_root(path: &Path) -> Option<PathBuf> {
        for parent in path.ancestors() {
            let config_path = parent.join(CONFIG_FILE);

            if config_path.exists() {
                return Some(parent.to_path_buf());
            }
        }

        None
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
}
