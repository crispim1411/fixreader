use serde::{Serialize, Deserialize};

use crate::FixSchema;

#[derive(Serialize, Deserialize)]
pub struct FixMsg {
    values: Vec<Field>,
    msg_schema: MessageSchema,
}

#[derive(Serialize, Deserialize)]
pub struct Field {
    tag: String,
    value: String,
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

pub struct FixConverter(pub FixSchema);

impl FixConverter {
    pub fn from_string(&self, input: &str, separator: &str) -> Result<FixMsg, &'static str> {

        if input.matches(separator).count() > 0  {
            let mut fields = input.split(separator)
                .take_while(|&element| !element.is_empty())
                .map(|p| {
                    match p.split_once('=') {
                        Some((tag, value)) => self.0.parse(tag, value),
                        None => ("Error".to_string(), p.to_string())
                    }
                })
                .map(|x| Field { tag: x.0, value: x.1 });
            let msg_type_field = fields.find(|field| field.tag == "MsgType").unwrap();
            let msg_schema = self.find_msg_schema(&msg_type_field.value)?;
            return Ok(FixMsg { values: fields.collect(), msg_schema });
        }
        return Err("Separador inválido");
    }

    fn find_msg_schema(&self, msgtype: &str) -> Result<MessageSchema, &'static str> {
        let search = 
            self.0.messages.values
            .iter()
            .find(|&item| item.msgtype == msgtype);
        println!("msgType: {msgtype}");
        match search {
            Some(msg) => {
                let msg = msg.clone();
                let msg = MessageSchema {
                    name: msg.name.clone(),
                    fields: msg.fields
                        .iter()
                        .map(|field| FieldSchema { name: field.name.clone(), required: field.required.clone() })
                        .collect()
                };
                Ok(msg)
            },
            None => Err("Message not found in schema")
        }
    }
}

