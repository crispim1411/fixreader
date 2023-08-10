use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct FixSchema {
    pub header: Header,
    pub trailer: Trailer,
    pub messages: Messages,
    pub components: Components,
    pub fields: Fields,
}

#[derive(Deserialize, Debug)]
pub struct Header {
    #[serde(rename = "$value")]
    pub values: Vec<FieldHeader>,
}

#[derive(Deserialize, Debug)]
pub struct Trailer {
    #[serde(rename = "$value")]
    pub values: Vec<FieldHeader>,
}

#[derive(Deserialize, Debug)]
pub struct Messages {
    #[serde(rename = "$value")]
    pub values: Vec<Message>,
}

#[derive(Deserialize, Debug)]
pub struct Components {
    #[serde(rename = "$value")]
    pub values: Vec<Component>,
}

#[derive(Deserialize, Debug)]
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

impl fmt::Debug for FieldHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.required)?;
        for field in self.group.iter() {
            write!(f, ",\n{} ({})", field.name, field.required)?;
        }
        Ok(())
    }
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

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {} {:#?}", self.msgtype, self.name, self.fields)
    }
}

#[derive(Deserialize, Debug)]
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

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {} [{}]", self.number, self.name, self.field_type)?;
        if !self.values.is_empty() {
            write!(f, "{:#?}", self.values)?;
        }
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct FieldValues {
    #[serde(rename = "@enum")]
    pub value: String,
    #[serde(rename = "@description")]
    pub description: String,
}

impl fmt::Debug for FieldValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.value, self.description)
    }
}
