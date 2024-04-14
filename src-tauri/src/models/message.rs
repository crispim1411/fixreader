use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FixMessage {
    pub values: Vec<FieldConverted>,
}

#[derive(Serialize, Deserialize)]
pub struct FieldConverted {
    tag: String,
    title: String,
    value: String,
    required: bool,
}

impl From<(String, String, String, bool)> for FieldConverted {
    fn from(value: (String, String, String, bool)) -> Self {
        FieldConverted { tag: value.0, title: value.1, value: value.2, required: value.3 }
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