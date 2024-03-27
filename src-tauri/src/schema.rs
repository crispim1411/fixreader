use std::collections::HashSet;
use serde::Deserialize;

use crate::AppError;

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

#[derive(Deserialize, Clone)]
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

type ParsedTag = Vec<(String, String)>;

impl FixSchema {
    pub fn parse_tags<'a, I>(&self, tag_values: I) -> Result<ParsedTag, AppError> 
    where 
        I: Iterator<Item=(&'a str, &'a str)> 
    {
        let mut values = Vec::new();
        let mut msgtype = "";

        for (tag, value) in tag_values {
            if tag == "35" { msgtype = value; }
            let Some(field) = self.find_field_by_tag(tag) else { 
                values.push((tag.into(), value.into()));
                continue;
            };
            let description: &str = {
                if let Some(field) = field.values.iter().find(|item| &item.value == value) {
                    &field.description
                } else { value }
            };
            
            values.push((field.name.to_string(), description.to_string()));
        }
        let Some(msg_schema) = self.find_msgtype(msgtype) else {
            return Err("Tipo de mensagem não suportado".into());
        };
        self.validate(&values, &msg_schema)?;
        return Ok(values);
    }

    fn find_msgtype(&self, msgtype: &str) -> Option<Message> {
        if let Some(msg) = self.messages.values
            .iter()
            .find(|&item| item.msgtype == msgtype) {
                let fields = msg.fields.clone();
                let grouped: Vec<FieldHeader> = fields.iter().flat_map(|i| i.group.clone()).collect();
                return Some(Message {
                    name: msg.name.clone(),
                    msgtype: msg.msgtype.clone(),
                    msgcat: msg.msgcat.clone(),
                    fields: [fields, grouped].concat(),
                });
        }
        None
    }

    fn find_field_by_tag(&self, tag: &str) -> Option<&Field> {
        self.fields.values
            .iter()
            .find(|item| &item.number == tag)
    }

    fn validate(&self, values: &Vec<(String, String)>, msg: &Message) -> Result<(), AppError> {
        let required_headers: Vec<&str> = self.header.values.iter().filter(|x| x.required == "Y")
            .map(|f| f.name.as_ref()).collect();
        let required_trailers: Vec<&str> = self.header.values.iter().filter(|x| x.required == "Y")
            .map(|f| f.name.as_ref()).collect();
        let required_fields: Vec<&str> = msg.fields.iter().filter(|x| x.required == "Y")
            .map(|f| f.name.as_ref()).collect();

        let mut missing_fields = HashSet::new();
        for required_header in required_headers {
            if values.iter().find(|v| v.0 == required_header).is_none() { 
                missing_fields.insert(required_header);
            }
        }
        for required_trailer in required_trailers {
            if values.iter().find(|v| v.0 == required_trailer).is_none() {
                missing_fields.insert(required_trailer);
            }
        }
        for required_field in required_fields {
            if values.iter().find(|v| v.0 == required_field).is_none() {
                missing_fields.insert(required_field);
            }
        } 
        if missing_fields.len() != 0 {
            return Err(format!("Missing fields {missing_fields:?}").into());
        }
        Ok(())
    }
}
