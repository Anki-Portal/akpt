use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "apiVersion")]
    api_version: u32,
    models: Option<Vec<ModelConfig>>,
}
impl Config {
    pub fn new(api_version: u32) -> Self {
        Self {
            api_version,
            models: None,
        }
    }
    pub fn add_model(&mut self, model: ModelConfig) -> Result<(), String> {
        if self.models.is_none() {
            self.models = Some(Vec::new());
        }

        if let Some(models) = self.models.as_mut() {
            if models.contains(&model) {
                return Err(format!("Model {} already exists", model.name));
            }

            models.push(model);
        }
        
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub id: String,
    pub fields: Vec<FieldConfig>,
}
impl PartialEq for ModelConfig {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Serialize, Deserialize)]
pub struct FieldConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: FieldType,
}

#[derive(Serialize, Deserialize)]
pub enum FieldType {
    Text,
    Img,
}
