use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FixMessage {
    pub values: Vec<TagValue>,
}

#[derive(Serialize, Deserialize)]
pub struct TagValue {
    pub tag: String,
    pub title: String,
    pub value: String,
}

impl From<(String, String, String)> for TagValue {
    fn from(value: (String, String, String)) -> Self {
        TagValue { tag: value.0, title: value.1, value: value.2 }
    }
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