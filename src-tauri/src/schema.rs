use serde::Deserialize;
use std::{fmt, slice::{Iter, IterMut}};

#[derive(Deserialize)]
pub struct FixSchema {
    pub header: Header,
    pub trailer: Trailer,
    pub messages: Messages,
    pub components: Components,
    pub fields: Fields,
}

#[derive(Deserialize)]
pub struct Header {
    #[serde(rename = "$value")]
    pub values: Vec<FieldHeader>,
}

#[derive(Deserialize)]
pub struct Trailer {
    #[serde(rename = "$value")]
    pub values: Vec<FieldHeader>,
}

#[derive(Deserialize)]
pub struct Messages {
    #[serde(rename = "$value")]
    pub values: Vec<Message>,
}

#[derive(Deserialize)]
pub struct Components {
    #[serde(rename = "$value")]
    pub values: Vec<Component>,
}

#[derive(Deserialize)]
pub struct Fields {
    #[serde(rename = "$value")]
    pub values: Vec<Field>,
}

#[derive(Deserialize)]
pub struct FieldHeader {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@required")]
    pub required: String,
    #[serde(default, rename = "$value")]
    pub group: Vec<FieldHeader>,
}

#[derive(Deserialize)]
pub struct Message {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@msgtype")]
    pub msgtype: String,
    #[serde(rename = "@msgcat")]
    pub msgcat: String,
    #[serde(default, rename = "$value")]
    pub fields: Vec<FieldHeader>,
}

#[derive(Deserialize)]
pub struct Component {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(default, rename = "$value")]
    pub fields: Vec<FieldHeader>,
}

#[derive(Deserialize)]
pub struct Field {
    #[serde(rename = "@number")]
    pub number: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub field_type: String,
    #[serde(default, rename = "$value")]
    pub values: Vec<FieldValues>,
}

#[derive(Deserialize)]
pub struct FieldValues {
    #[serde(rename = "@enum")]
    pub value: String,
    #[serde(rename = "@description")]
    pub description: String,
}

impl FixSchema {
    pub fn parse_tags<'a>(&self, mut tag_values: impl Iterator<Item=(&'a str, &'a str)>) -> Result<Vec<(String, String, String)>, &'static str> {
        let msgtype_field = tag_values.find(|x| x.0 == "35").expect("Mensagem fix inválida");
        let msg_schema = self.find_msg_schema(msgtype_field.1).expect("Tipo de mensagem não suportado");

        let mut values = Vec::new();
        for (tag, value) in tag_values {
            if let Some(field) = self.fields.values.iter().find(|item| &item.number == tag) {
                let value: &str = {
                    if let Some(field) = field.values.iter().find(|item| &item.value == value) {
                        &field.description
                    } else { value }
                };
                let required = 
                    match msg_schema.fields.iter().find(|x| x.name == field.name) {
                        Some(schema_field) => schema_field.required.clone(),
                        None => String::new()
                    };
                
                values.push((field.name.to_string(), value.to_string(), required));
            } else { 
                values.push((tag.to_string(), value.to_string(), String::new()));
            }
        }
        return Ok(values);
    }

    pub fn find_msg_schema(&self, msgtype: &str) -> Option<&Message> {
        self.messages.values
            .iter()
            .find(|&item| item.msgtype == msgtype)
    }
}
