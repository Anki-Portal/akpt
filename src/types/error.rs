use std::{error::Error, fmt::Display};

use proc_macros::RequestError;

pub trait RequestError: Error {}

#[derive(Debug, RequestError)]
pub struct NullResponseError;

impl Display for NullResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Null response error")
    }
}

#[derive(Debug, RequestError)]
pub struct APIError {
    pub message: String,
}

impl Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API error: {}", self.message)
    }
}