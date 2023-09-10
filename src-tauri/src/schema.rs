use serde::Deserialize;
use std::fmt;

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
    pub fn parse(&self, tag: &str, value: &str) -> (String, String) {
        if let Some(field) = self.fields.values.iter().find(|item| item.number == tag) {
            let value = {
                if let Some(field) = field.values.iter().find(|item| item.value == value) {
                    &field.description
                } else {
                    value
                }
            };
            return (field.name.clone(), value.to_string());
        }
        (tag.to_string(), value.to_string())
    }
}
