use anyhow::Result;
use quick_xml::de::{from_reader, DeError};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

struct FixMessage;

// #[derive(Deserialize, Debug)]
// struct FixSchema {
//     fix: SchemaItems,
// }

#[derive(Deserialize, Debug)]
struct FixSchema {
    header: SchemaItem,
    trailer: SchemaItem,
    messages: SchemaItem,
    components: SchemaItem,
    fields: SchemaItem,
}

#[derive(Deserialize, Debug)]
enum SchemaItem {
    #[serde(rename = "header")]
    Header {
        #[serde(rename = "$value")]
        field: Vec<FieldHeader>,
    },
    #[serde(rename = "trailer")]
    Trailer {
        #[serde(rename = "$value")]
        field: Vec<FieldHeader>,
    },
    #[serde(rename = "messages")]
    Messages {
        #[serde(rename = "$value")]
        message: Vec<Message>,
    },
    #[serde(rename = "components")]
    Components {
        #[serde(rename = "$value")]
        field: Vec<Component>,
    },
    #[serde(rename = "fields")]
    Fields {
        #[serde(rename = "$value")]
        field: Vec<Field>,
    },
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
    number: usize,
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

fn main() -> Result<()> {
    /*let mut reader = Reader::from_file(Path::new("FIX44RFQ.xml"))?;

    let mut count = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ev)) => {
                let attributes: Vec<String> = ev
                    .attributes()
                    .map(|att| {
                        std::str::from_utf8(att.unwrap().value.as_ref())
                            .unwrap()
                            .into()
                    })
                    .collect();
                println!(
                    "{:?}",
                    std::str::from_utf8(ev.local_name().into_inner()).unwrap()
                );
                println!("Attributes: {:?}", attributes);
            }
            Ok(Event::Text(t)) => txt.push(t.unescape().unwrap().into_owned()),
            Ok(Event::Eof) => break,
            Err(err) => println!("Error on position {}: {}", reader.buffer_position(), err),
            _ => (),
        }
        buf.clear();
    }
    */
    let mut reader = Reader::from_file(&Path::new("FIX44RFQ.xml"))?;
    let schema: FixSchema = quick_xml::de::from_reader(reader.into_inner())?;
    println!("Output: {:#?}", schema);
    Ok(())
}
