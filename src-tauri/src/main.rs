// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fixreader::FixSchema;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use tauri::{State, Manager, App};
use std::{fs::File, path::Path};

const SEPARATOR: &str = "^";

#[derive(Deserialize, Serialize, Default)]
struct MyConfig {
    schema_path: String,
}

trait MyMethods {
    fn load_files(&mut self) -> Result<(String, FixSchema), String>;
}

impl MyMethods for App {
    fn load_files(&mut self) -> Result<(String, FixSchema), String> {
        let Some(config_file) = self.path_resolver().resolve_resource("config.json") else {
            return Err("config.json not found".into());
        };
        let Ok(filename) =  File::open(&config_file) else {
            return Err("Error trying read file".into());
        };
        let config: MyConfig = serde_json::from_reader(filename).unwrap();
        let Ok(reader) = Reader::from_file(Path::new(&config.schema_path)) else {
            return Err("schema not found".into());
        };
        let Ok(schema) = quick_xml::de::from_reader(reader.into_inner()) else {
            return Err("Error reading schema".into());
        };
        return Ok((config.schema_path, schema));
    }
}

#[derive(Debug)]
struct Context {
    file: String,
    schema: Option<FixSchema>,
}

#[tauri::command]
fn ping(state: State<Context>) -> String {
    return state.file.clone();
}

#[tauri::command]
fn read_fix(state: State<Context>, input: &str) -> Vec<(String, String)> {
    let Some(schema) = &state.schema else {
        panic!("Schema not found in context");
    };
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
    tauri::Builder::default()
        .setup(|app| {
            match app.load_files() {
                Ok((file, schema)) => {
                    app.manage(Context { file, schema: Some(schema)});
                }
                Err(e) => {
                    app.manage(Context { file: e, schema: None});
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![read_fix, ping])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
