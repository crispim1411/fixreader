use serde::{Serialize, Deserialize};

use crate::FixSchema;

#[derive(Serialize, Deserialize)]
pub struct FixMsg {
    fields: Vec<Field>,
}

#[derive(Serialize, Deserialize)]
pub struct Field {
    tag: String,
    value: String,
}

pub struct FixConverter(pub FixSchema);

impl FixConverter {
    pub fn from_string(&self, input: &str, separator: &str) -> Result<FixMsg, &'static str> {

        if input.matches(separator).count() > 0  {
            let fields: Vec<Field> = input.split(separator)
                .take_while(|&element| !element.is_empty())
                .map(|p| {
                    match p.split_once('=') {
                        Some((tag, value)) => self.0.parse(tag, value),
                        None => ("Error".to_string(), p.to_string())
                    }
                })
                .map(|x| Field { tag: x.0, value: x.1 })
                .collect();
            return Ok(FixMsg { fields });
        }
        return Err("Separador inválido");
    }
}

