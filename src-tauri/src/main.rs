// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fixreader::FixSchema;
use quick_xml::Reader;
use std::path::Path;

#[tauri::command]
fn read_fix(input: &str) -> Vec<(String, String)> {
    let Ok(reader) = Reader::from_file(Path::new("FIX44RFQ.xml")) else {
        return vec![];
    };
    let schema: FixSchema = quick_xml::de::from_reader(reader.into_inner()).unwrap();

    let separator = "^";

    let result: Vec<(String, String)> = input
        .split(separator)
        .take_while(|&element| !element.is_empty())
        .map(|p| {
            match p.split_once('=') {
                Some((tag, value)) => schema.parse(tag, value),
                None => ("Error".to_string(), p.to_string())
            }
        })
        .collect();
    return result;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_fix])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
