use std::io;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppError(String);

impl From<&str> for AppError {
    fn from(error: &str) -> Self {
        AppError(error.into())
    }
}

impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError(error)
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError(error.to_string())
    }
}   

impl From<quick_xml::Error> for AppError {
    fn from(error: quick_xml::Error) -> Self {
        AppError(error.to_string())
    }
}

impl From<quick_xml::DeError> for AppError {
    fn from(error: quick_xml::DeError) -> Self {
        AppError(error.to_string())
    }
}