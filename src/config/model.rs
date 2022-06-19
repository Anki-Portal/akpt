use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Eq)]
pub struct ModelConfig {
    pub name: String,
    pub id: String,
    pub cloze: Option<bool>,
}
impl PartialEq for ModelConfig {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}