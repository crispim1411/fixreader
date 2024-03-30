use std::collections::HashSet;
use std::path::Path;

use quick_xml::Reader;

use crate::models::*;
use crate::services::Cache;

const DELIMITER_1: &'static str = "\x01";
const DELIMITER_2: &'static str = "|";

pub struct FixConverter {
    pub schema: Option<FixSchema>,
    pub filename: String,
}

impl FixConverter {
    pub fn new() -> Self {
        FixConverter { schema: None, filename: "".into() }
    }
    
    pub fn try_load(&mut self) -> Result<(), AppError> {
        let schema_file = Cache::load()?;
        return self.load_from(&schema_file);
    }

    pub fn load_from(&mut self, file_path: &str) -> Result<(), AppError> {
        let schema = FixConverter::read_from_xml(&file_path)?;
        self.schema = Some(schema);
        self.filename = file_path.into();
        Ok(())
    }


    pub fn read_from_xml(schema_file: &str) -> Result<FixSchema, AppError> {
        let reader = Reader::from_file(Path::new(schema_file))?;
        let schema = quick_xml::de::from_reader(reader.into_inner())?;
        Ok(schema)
    }

    pub fn from_str(&self, input: &str) -> Result<FixMessage, AppError> {
        let delimiter = 
            if input.contains(DELIMITER_1) { DELIMITER_1 } 
            else if input.contains(DELIMITER_2) { DELIMITER_2 }
            else {
                return Err("Separador inválido".into());
            };

        let tag_values = input
            .split(delimiter)
            .take_while(|&element| !element.is_empty())
            .filter_map(|x| x.split_once('='));

        let values = self.parse_tags(tag_values);

        let validate = false;
        if validate {
            // let Some(msg_type) = tag_values.filter(|t| t.0 == "35").next() else {
            //     return Err("MessageType não presente na mensagem!".into());
            // };
            // println!("MsgType: {:?}", msg_type);
            // let Some(msg_schema) = self.find_msgtype(msg_type.1) else {
            //     return Err("Tipo de mensagem não suportado".into());
            // };
            // self.validate(&values, &msg_schema)?;
        }   

        return Ok(FixMessage { values });
    }    

    fn parse_tags<'a, I>(&self, tag_values: I) -> Vec<TagValue>
    where 
        I: Iterator<Item=(&'a str, &'a str)> 
    {    
        tag_values
            .map(|(tag, value)| {
                if let Some(field) = self.find_field_by_tag(tag) {
                    if let Some(description) = self.find_field_description(value, field) {
                        return (field.name.clone(), description);
                    }
                    return (field.name.to_string(), value.to_string());
                }
                return (tag.to_string(), value.to_string());
            })
            .map(|(tag, value)| TagValue { tag, value })
            .collect()
    }

    fn find_msgtype(&self, msgtype: &str) -> Option<Message> {
        let Some(schema) = &self.schema else { 
            panic!("Tentativa de parsear sem schema carregado!");
        };

        if let Some(msg) = schema.messages.values
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
        let Some(schema) = &self.schema else { 
            panic!("Tentativa de parsear sem schema carregado!");
        };
        return schema.fields.values
            .iter()
            .find(|item| &item.number == tag);
    }

    fn find_field_description(&self, key: &str, field: &Field) -> Option<String> {
        if let Some(field) = field.values.iter().find(|item| &item.value == key) {
            return Some(field.description.clone());
        }  
        return None;
    }

    fn validate(&self, values: &Vec<(String, String)>, msg: &Message) -> Result<(), AppError> {
        let Some(schema) = &self.schema else { 
            panic!("Tentativa de parsear sem schema carregado!");
        };

        let required_headers: Vec<&str> = schema.header.values.iter().filter(|x| x.required == "Y")
            .map(|f| f.name.as_ref()).collect();
        let required_trailers: Vec<&str> = schema.header.values.iter().filter(|x| x.required == "Y")
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
