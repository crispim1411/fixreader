mod schema;
mod converter;

use std::error::Error;

pub use schema::FixSchema;
pub use converter::FixMsg;

pub type AppResult<T> = std::result::Result<T, &'static str>;
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;