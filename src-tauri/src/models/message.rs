use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FixMessage {
    pub values: Vec<TagValue>,
}

#[derive(Serialize, Deserialize)]
pub struct TagValue {
    pub tag: String,
    pub value: String,
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