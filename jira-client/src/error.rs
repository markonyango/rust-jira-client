use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
  #[error("Generic error: {0}")]
  GenericError(String)
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        AppError::GenericError(value.to_string())
    }
}
