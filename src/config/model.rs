use serde::{Serialize, Deserialize};

use super::field::FieldConfig;

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