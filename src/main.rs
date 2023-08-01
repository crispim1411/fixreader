use anyhow::Result;
use quick_xml::Reader;
use serde::Deserialize;
use std::{fs::File, io::Read, path::Path};

#[derive(Deserialize, Debug)]
struct FixSchema {
    header: Header,
    trailer: Trailer,
    messages: Messages,
    components: Components,
    fields: Fields,
}

#[derive(Deserialize, Debug)]
struct Header {
    #[serde(rename = "$value")]
    values: Vec<FieldHeader>,
}

#[derive(Deserialize, Debug)]
struct Trailer {
    #[serde(rename = "$value")]
    values: Vec<FieldHeader>,
}

#[derive(Deserialize, Debug)]
struct Messages {
    #[serde(rename = "$value")]
    values: Vec<Message>,
}

#[derive(Deserialize, Debug)]
struct Components {
    #[serde(rename = "$value")]
    values: Vec<Component>,
}

#[derive(Deserialize, Debug)]
struct Fields {
    #[serde(rename = "$value")]
    values: Vec<Field>,
}

#[derive(Deserialize, Debug)]
struct FieldHeader {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@required")]
    required: String,
    #[serde(default, rename = "$value")]
    group: Vec<FieldHeader>,
}

#[derive(Deserialize, Debug)]
struct Message {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@msgtype")]
    msgtype: String,
    #[serde(rename = "@msgcat")]
    msgcat: String,
    #[serde(default, rename = "$value")]
    fields: Vec<FieldHeader>,
}

#[derive(Deserialize, Debug)]
struct Component {
    #[serde(rename = "@name")]
    name: String,
    #[serde(default, rename = "$value")]
    fields: Vec<FieldHeader>,
}

#[derive(Deserialize, Debug)]
struct Field {
    #[serde(rename = "@number")]
    number: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    field_type: String,
    #[serde(default, rename = "$value")]
    values: Vec<FieldValues>,
}

#[derive(Deserialize, Debug)]
struct FieldValues {
    #[serde(rename = "@enum")]
    value: String,
    #[serde(rename = "@description")]
    description: String,
}

enum Search<'a> {
    Field(Option<&'a Field>),
    Message(Option<&'a Message>),
    None,
}

fn main() -> Result<()> {
    let reader = Reader::from_file(Path::new("FIX44RFQ.xml"))?;
    let schema: FixSchema = quick_xml::de::from_reader(reader.into_inner())?;

    let mut buf = String::new();
    let mut file = File::open("msg.txt")?;
    file.read_to_string(&mut buf)?;
    println!("msg: {}", buf);

    let separator = "|";
    let pieces: Vec<String> = buf
        .split(separator)
        .take_while(|&element| !element.is_empty())
        .map(|p| p.split_once('=').unwrap_or((p, p)))
        .map(|(tag, value)| format!("{} = {}", search_tag(&schema, tag).unwrap_or(tag), value)) 
        .collect();
    println!("{:#?}", pieces);

    Ok(())
}

fn search_tag<'a>(schema: &'a FixSchema, tag: &str) -> Option<&'a str> {
    // search fields
    if let Some(field) = schema.fields.values.iter().find(|item| item.number == tag) {
        return Some(&field.name);
    }
    None
}

#[allow(dead_code)]
fn iterative_mode(schema: &FixSchema) -> Result<()> {
    loop {
        let stdin = std::io::stdin();

        println!(
            "Choose search (default=1):
            \r 0 - field by name
            \r 1 - field by tag
            \r 2 - message by name
            \r 3 - message by msgtypes\n"
            );
        let search_mode = {
            let mut buf = String::new();
            stdin.read_line(&mut buf)?;
            buf.trim().to_string()
        };

        println!("Enter key:");
        let key = {
            let mut buf = String::new();
            stdin.read_line(&mut buf)?;
            buf.trim().to_string()
        };

        let found = {
            match search_mode.as_ref() {
                "0" => Search::Field(schema.fields.values.iter().find(|item| item.name == key)),
                "1" => Search::Field(schema.fields.values.iter().find(|item| item.number == key)),
                "2" => Search::Message(schema.messages.values.iter().find(|item| item.name == key)),
                "3" => Search::Message(
                    schema
                    .messages
                    .values
                    .iter()
                    .find(|item| item.msgtype == key),
                    ),
                _ => Search::None,
            }
        };
        match found {
            Search::Field(Some(field)) => println!("{:#?}\n", field),
            Search::Message(Some(message)) => println!("{:#?}\n", message),
            _ => println!("Tag not found\n"),
        }
    }
}

