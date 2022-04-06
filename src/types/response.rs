use serde::Deserialize;

use super::error::{RequestError, APIError, NullResponseError};

pub trait ResponseData {}

#[derive(Deserialize)]
pub struct Response<T: ResponseData> {
    result: Option<T>,
    error: Option<String>,
}

impl<T> Response<T> where T: ResponseData{
    pub fn ok(self) -> Result<T, Box<dyn RequestError>> {
        if let Some(error) = self.error {
            return Err(Box::new(APIError { message: error }))
        }

        match self.result {
            Some(result) => Ok(result),
            None => Err(Box::new(NullResponseError)),
        }        
    }
}