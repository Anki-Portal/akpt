use proc_macros::ErrDisplay;

#[derive(Debug, ErrDisplay)]
pub struct CustomError {
    pub message: String,
}

impl CustomError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}