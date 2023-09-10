// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fixreader::{FixSchema, FixConverter, FixMsg};
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use tauri::{State, Manager, App};
use std::{fs::File, path::Path, sync::Mutex};

struct Context(Mutex<AppState>);

enum AppState {
    Loaded { file_loaded: String },
    Loading,
    Unloaded { error: String }
}

#[tauri::command]
fn get_schema_file(state: State<Context>) -> Result<String, &'static str> {
    let state = state.0.lock().expect("Error reading app state");
    if let AppState::Loaded { file_loaded, .. } = &*state {
        return Ok(file_loaded.clone());
    }
    return Err("No file");
}

#[tauri::command]
fn read_fix(converter: State<FixConverter>, input: &str, separator: &str) -> Result<FixMsg, &'static str> {
    return converter.from_string(input, separator);
}

#[derive(Deserialize, Serialize, Default)]
struct MyConfig {
    schema_path: String,
}
//

fn load(app: &mut App) -> Result<(String, FixSchema), String> {
    let schema_file = load_config(app)?;
    let schema = load_from_xml(schema_file.clone())?;
    return Ok((schema_file, schema));
}

//// Mover para dialogo escolhendo xml se não encontrar
fn load_config(app: &mut App) -> Result<String, String> {
    let Some(config_file) = app.path_resolver().resolve_resource("config.json") else {
        return Err("config.json not found".into());
    };
    let Ok(filename) =  File::open(&config_file) else {
        return Err("Error trying read file".into());
    };
    let config: MyConfig = serde_json::from_reader(filename).unwrap();
    return Ok(config.schema_path);
}

fn load_from_xml(schema_file: String) -> Result<FixSchema, String> {
    let Ok(reader) = Reader::from_file(Path::new(&schema_file)) else {
        return Err("schema not found".into());
    };
    let Ok(schema) = quick_xml::de::from_reader(reader.into_inner()) else {
        return Err("Error reading schema".into());
    };
    return Ok(schema);
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {           
            match load(app) {
                Ok((file, schema)) => {
                    let app_state = AppState::Loaded { file_loaded: file };
                    app.manage(Context(Mutex::new(app_state)));
                    app.manage(FixConverter(schema));
                }
                Err(e) => {
                    let app_state = AppState::Unloaded { error: e };
                    app.manage(Context(Mutex::new(app_state)));
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![read_fix, get_schema_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 
