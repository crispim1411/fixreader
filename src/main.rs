use anyhow::Result;
use clap::Parser;
use quick_xml::Reader;
use std::{io, path::Path};

use fixreader::{Field, FixSchema, Message};

#[derive(Parser, Debug)]
struct Arg {
    #[clap(value_enum)]
    mode: Mode,
}

#[derive(Clone, clap::ValueEnum, Debug)]
enum Mode {
    Fix,
    Search,
}

enum Search<'a> {
    Field(Option<&'a Field>),
    Message(Option<&'a Message>),
    None,
}

fn main() -> Result<()> {
    let arg = Arg::parse();

    let reader = Reader::from_file(Path::new("FIX44RFQ.xml"))?;
    let schema: FixSchema = quick_xml::de::from_reader(reader.into_inner())?;

    let _ = match arg.mode {
        Mode::Fix => fixread_mode(&schema),
        Mode::Search => iterative_mode(&schema),
    };

    Ok(())
}

fn fixread_mode(schema: &FixSchema) -> Result<()> {
    println!("Fix parsing mode");

    let separator = "|";

    let stdin = io::stdin();
    for line in stdin.lines() {
        let line = line.expect("Expect line");

        let (oks, errors): (Vec<_>, Vec<_>) = line
            .split(separator)
            .take_while(|&element| !element.is_empty())
            .map(|p| {
                match p.split_once('=') {
                    Some((tag, value)) => {
                        let (tag, value) = parse_tag(&schema, tag, value);
                        return Ok(format!("{tag} = {value}"));
                    }
                    None => { 
                        return Err(format!("Error parsing the item '{}'", p)); 
                    }
                }
            })
            .partition(|result| result.is_ok());

        let formatted: Vec<_> = oks.into_iter().map(Result::unwrap).collect();
        let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();

        println!("{:#?}", formatted);

        for error in errors {
            println!("{}", error);
        }
    }
    Ok(())
}

fn parse_tag<'a>(schema: &'a FixSchema, tag: &'a str, value: &'a str) -> (&'a str, &'a str) {
    match schema.fields.values.iter().find(|item| item.number == tag) {
        Some(field) => {
            let value = 
                match field.values.iter().find(|item| item.value == value) {
                    Some(field) => &field.description,
                    None => value,
            };
            return (tag, value);
        }
        None => (tag, value)
    }
}

fn iterative_mode(schema: &FixSchema) -> Result<()> {
    println!("Search tag mode");

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
