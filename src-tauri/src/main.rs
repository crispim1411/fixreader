// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fixreader::FixSchema;
use quick_xml::Reader;
use tauri::{State, Manager};
use std::path::Path;

const SEPARATOR: &str = "^";

#[derive(Debug)]
struct Context(FixSchema);

#[tauri::command]
fn ping() {
    println!("Pong");
}

#[tauri::command]
fn read_fix(state: State<Context>, input: &str) -> Vec<(String, String)> {
    let schema = &state.0;
    let result: Vec<(String, String)> = input
        .split(SEPARATOR)
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
    let file_path = "FIX44RFQ.xml";
    tauri::Builder::default()
        .setup(|app| {
            println!("Starting...");
            let reader = Reader::from_file(Path::new(file_path)).unwrap();
            let schema: FixSchema = quick_xml::de::from_reader(reader.into_inner()).unwrap();
            println!("Loaded");
            app.manage(Context(schema));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![read_fix, ping])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
