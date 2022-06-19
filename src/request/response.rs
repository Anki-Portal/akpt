use std::collections::HashMap;

use serde::de::DeserializeOwned;

use self::response_types::{ModelFieldNames, ModelNamesAndIds, ModelStyling, ModelTemplates};

pub mod response_types;

pub trait AnkiResponse: DeserializeOwned {}

macro_rules! impl_anki_response {
    ($($t:ty),+) => {
        $(
            impl AnkiResponse for $t {}
        )+
    };
}

impl_anki_response!(
    ModelNamesAndIds,
    ModelFieldNames,
    ModelTemplates,
    ModelStyling
);

pub fn parse_response<T: AnkiResponse>(response: String) -> T {
    serde_json::from_str(&response).unwrap()
}
