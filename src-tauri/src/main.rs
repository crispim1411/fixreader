// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fixreader::FixSchema;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use tauri::{State, Manager};
use std::{path::Path, fs::File};

const SEPARATOR: &str = "^";

#[derive(Debug)]
struct Context {
    file: String,
    schema: FixSchema,
}
    

#[tauri::command]
fn ping(state: State<Context>) -> String {
    return state.file.clone();
}

#[tauri::command]
fn read_fix(state: State<Context>, input: &str) -> Vec<(String, String)> {
    let schema = &state.schema;
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

#[derive(Deserialize, Serialize, Default)]
struct Config {
    schema_path: String,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let config_file = app.path_resolver().resolve_resource("config.json").expect("Not found config.json");
            println!("File: {:?}", config_file.as_path());
            println!("AppDir: {:?}", app.path_resolver().app_config_dir().unwrap());
            let config: Config = serde_json::from_reader(File::open(&config_file).unwrap()).expect("Error reading config.json");
            println!("Starting...");
            println!("Path: {}", &config.schema_path);
            let reader = Reader::from_file(Path::new(&config.schema_path)).expect("schema not found");
            let schema: FixSchema = quick_xml::de::from_reader(reader.into_inner()).expect("Error reading schema");
            println!("Loaded");
            app.manage(Context { file: config.schema_path, schema});
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![read_fix, ping])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
