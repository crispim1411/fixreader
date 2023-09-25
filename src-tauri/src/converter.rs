use serde::{Serialize, Deserialize};

use crate::{FixSchema, AppResult};

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

impl FixSchema {
    pub fn from_string(&self, input: &str, separator: &str) -> AppResult<FixMsg> {

        if input.matches(separator).count() > 0  {
            let tag_values = input.split(separator)
                .take_while(|&element| !element.is_empty())
                .map(|x| {
                    let Some(splitted) =  x.split_once('=') else {
                        return ("Error splitting ", x);
                    };
                    splitted
                }).into_iter();
            let values = self.parse_tags(tag_values).expect("Erro parseando tags");
            let values = values
                .into_iter()
                .map(|(tag, value, required)| Field { tag, value, required})
                .collect();

            return Ok(FixMsg { values });
        }
        return Err("Separador inválido".into());
    }    
}

