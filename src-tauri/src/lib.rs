mod schema;
mod converter;
pub use schema::FixSchema;
pub use converter::FixMsg;

use std::{error::Error, fs};
use lazy_static::lazy_static;

pub type AppResult<T> = std::result::Result<T, String>;
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

const CACHE_PATH: &'static str = "cache.txt";

lazy_static! {
    static ref FOLDER_PATH: String = {
        let app_data: String = std::env::var("APPDATA").expect("No APP_DATA directory");
        format!("{app_data}/fixreader")
    };
}

pub struct Cache;

impl Cache {
   pub  fn save(data: &str) -> Result<()>  {
        fs::create_dir_all(&*FOLDER_PATH).expect("Error creating cache folder on appdata");
        fs::write(CACHE_PATH, data)?;  
        Ok(())
    }

    pub fn load() -> Result<String> {
        let filepath = format!("{}/{}", *FOLDER_PATH, CACHE_PATH);
        let data = fs::read_to_string(filepath)?;
        Ok(data)
    }
}