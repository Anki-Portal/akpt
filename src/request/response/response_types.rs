use std::collections::HashMap;

use serde::Deserialize;

pub type ModelNamesAndIds = HashMap<String, String>;

pub type ModelFieldNames = Vec<String>;

pub type ModelTemplates = HashMap<String, ModelTemplate>;

#[derive(Debug, Deserialize)]
pub struct ModelTemplate {
    #[serde(rename = "Front")]
    pub front: String,
    #[serde(rename = "Back")]
    pub back: String,
}

#[derive(Debug, Deserialize)]
pub struct ModelStyling {
    pub css: String,
}