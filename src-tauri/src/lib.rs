mod schema;
mod converter;
pub use schema::FixSchema;
pub use converter::FixMsg;

use std::{error::Error, fs};

pub type AppResult<T> = std::result::Result<T, String>;
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

const CACHE_PATH: &'static str = "cache.txt";

pub struct Cache;

impl Cache {
   pub  fn save(data: &str) -> Result<()>  {
        fs::write(CACHE_PATH, data)?;  
        Ok(())
    }
    pub fn load() -> Result<String> {
        let data = fs::read_to_string(CACHE_PATH)?;
        Ok(data)
    }
}