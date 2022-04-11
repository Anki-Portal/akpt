use serde::{Serialize, Deserialize};

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
