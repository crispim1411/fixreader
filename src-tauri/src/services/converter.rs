use std::path::Path;
use quick_xml::Reader;

use crate::models::*;
use crate::services::Cache;

const DELIMITERS: [&'static str; 4] = ["|", "^A ", "\x01", "\\x01"];

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
        let delimiter = FixConverter::get_delimiter(input)?;
        
        let tag_values = input
            .split(delimiter)
            .take_while(|&element| !element.is_empty())
            .filter_map(|x| x.split_once('='));

        let msg_schema = 
            match tag_values.clone().find(|x| x.0 == "35") {
                Some(item) => self.find_msgtype(item.1),
                None => None
            };

        let values = self.parse_tags(tag_values, msg_schema.as_ref());

        return Ok(FixMessage { values });
    }

    fn get_delimiter(input: &str) -> Result<&str, AppError> {
        for delimiter in DELIMITERS {
            if input.contains(delimiter) {
                return Ok(delimiter);
            }
        }
        return Err("Separador inválido".into());
    }

    fn parse_tags<'a, I>(&self, tag_values: I, msg_schema: Option<&Message>) -> Vec<FieldConverted>
    where 
        I: Iterator<Item=(&'a str, &'a str)> 
    {    
        tag_values
            .map(|(tag, value)| {
                let mut field_name = String::new();
                let mut description = value.to_string();
                let mut requirement = false;
                if let Some(field) = self.find_field_by_tag(tag) {
                    if let Some(field_description) = FixConverter::find_field_description(value, field) {
                        description = field_description;
                    }
                    if let Some(msg_schema) = msg_schema {
                        requirement = self.find_requirement(&field, msg_schema);
                    }
                    field_name = field.name.clone();
                }
                return (tag.to_string(), field_name, description, requirement);
            })
            .map(|x| FieldConverted::from(x))
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

    fn find_field_description(key: &str, field: &Field) -> Option<String> {
        if let Some(field) = field.values.iter().find(|item| &item.value == key) {
            return Some(field.description.clone());
        }  
        return None;
    }

    fn find_requirement(&self, field: &Field, msg_schema: &Message) -> bool {
        let Some(schema) = &self.schema else { 
            panic!("Tentativa de parsear sem schema carregado!");
        };

        if let Some(header_field) = schema.header.values.iter().find(|x| x.name == field.name) {
            return header_field.required == "Y";
        }
        if let Some(trailer_field) = schema.trailer.values.iter().find(|x| x.name == field.name) {
            return trailer_field.required == "Y";
        }
        if let Some(field) = msg_schema.fields.iter().find(|x| x.name == field.name) {
            return field.required == "Y";
        }

        return false;
    }
}
