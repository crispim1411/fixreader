use serde::{Serialize, Deserialize};

use crate::FixSchema;

#[derive(Serialize, Deserialize)]
pub struct FixMsg {
    values: Vec<Field>,
}

#[derive(Serialize, Deserialize)]
pub struct Field {
    tag: String,
    value: String,
    required: String,
}

#[derive(Serialize, Deserialize)]
pub struct MessageSchema {
    name: String,
    fields: Vec<FieldSchema>
}

#[derive(Serialize, Deserialize)]
pub struct FieldSchema {
    name: String,
    required: String,
}

pub struct FixConverter {
    pub schema: FixSchema
}

impl FixConverter {
    pub fn from_string(&self, input: &str, separator: &str) -> Result<FixMsg, &'static str> {

        if input.matches(separator).count() > 0  {
            let mut tag_values = input.split(separator)
                .take_while(|&element| !element.is_empty())
                .map(|x| {
                    match x.split_once('=') {
                        Some((tag, value)) => (tag, value),
                        None => ("Error", x)
                    }
                }).into_iter();
            let values = self.schema.parse_tags(tag_values).expect("Erro parseando tags");
            let values = values
                .into_iter()
                .map(|(tag, value, required)| Field { tag, value, required})
                .collect();

            return Ok(FixMsg { values });
        }
        return Err("Separador inválido");
    }

    
}

