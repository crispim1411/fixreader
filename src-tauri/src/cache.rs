use std::fs;
use lazy_static::lazy_static;

use crate::AppError;

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