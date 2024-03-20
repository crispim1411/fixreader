mod schema;
mod converter;
pub use schema::FixSchema;
pub use converter::FixMsg;
use serde::{Deserialize, Serialize, Serializer};
use tauri::InvokeError;

use std::{error::Error, fs, io};
use lazy_static::lazy_static;

#[derive(Serialize, Deserialize)]
pub struct AppError(String);

impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError(error.to_string())
    } 
}

impl From<&str> for AppError {
    fn from(error: &str) -> Self {
        AppError(error.into())
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

//pub type AppResult<T> = std::result::Result<T, String>;
// pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

const CACHE_FILENAME: &'static str = "cache.txt";

lazy_static! {
    static ref FOLDER_PATH: String = {
        let app_data: String = std::env::var("APPDATA").expect("No APP_DATA directory");
        format!("{app_data}/fixreader")
    };
}

pub struct Cache;

impl Cache {
   pub  fn save(data: &str) -> Result<(), AppError>  {
        let folder_path = &*FOLDER_PATH;
        fs::create_dir_all(&folder_path).expect("Error creating cache folder on appdata");
        let cache_path = format!("{folder_path}/{CACHE_FILENAME}");
        fs::write(cache_path, data)?;  
        Ok(())
    }

    pub fn load() -> Result<String, AppError> {
        let folder_path = &*FOLDER_PATH;
        let cache_path = format!("{folder_path}/{CACHE_FILENAME}");
        let data = fs::read_to_string(cache_path)?;
        Ok(data)
    }
}