// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fixreader::FixSchema;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use tauri::{State, Manager, App, AppHandle, Error};
use std::{fs::File, path::Path, sync::Mutex};

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
struct Schema(Option<FixSchema>);
struct FileLoaded(String);
struct ErrorMsg(String);

#[derive(Serialize, Deserialize)]
struct FixMsg {
    fields: Vec<Field>,
}

#[derive(Serialize, Deserialize)]
struct Field {
    tag: String,
    value: String,
}

#[tauri::command]
fn get_schema_file(state: State<FileLoaded>) -> String {
    return state.0.clone();
}

#[tauri::command]
fn read_fix(state: State<Schema>, input: &str, separator: &str) -> FixMsg {
    let Some(schema) = &state.0 else {
        panic!("Schema not found in context");
    };
    let fields = input
        .split(separator)
        .take_while(|&element| !element.is_empty())
        .map(|p| {
            match p.split_once('=') {
                Some((tag, value)) => schema.parse(tag, value),
                None => ("Error".to_string(), p.to_string())
            }
        })
        .map(|x| Field { tag: x.0, value: x.1 })
        .collect();
    return FixMsg { fields };
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {           
            match app.load_files() {
                Ok((file, schema)) => {
                    app.manage(Schema(Some(schema)));
                    app.manage(FileLoaded(file));
                }
                Err(e) => {
                    app.manage(ErrorMsg(e));
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![read_fix, get_schema_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 
