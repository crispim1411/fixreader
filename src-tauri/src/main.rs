// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fixreader::{FixSchema, FixMsg, AppResult, Result};
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use tauri::{State, Manager, App, AppHandle};
use std::{fs::File, path::Path, sync::Mutex};

struct Context(Mutex<AppState>);

enum AppState {
    Loaded { 
        file_loaded: String, 
        schema: FixSchema, 
    },
    Unloaded { error: String },
}

#[tauri::command]
fn get_schema_file(state: State<Context>) -> AppResult<String> {
    let state = state.0.lock().expect("Error reading app state");
    if let AppState::Loaded { file_loaded, .. } = &*state {
        return Ok(file_loaded.clone());
    }
    return Err("No file".into());
}

#[tauri::command]
fn set_schema_file(context: State<Context>, path: &str)  -> AppResult<()> {
    let mut state = context.0.lock().unwrap();
    let Ok(schema) = load_from_xml(path) else {
        return Err("Error reading schema file")?;
    };
    *state = AppState::Loaded { file_loaded: path.into(), schema };
    Ok(())
}

#[tauri::command]
fn read_fix(context: State<Context>, input: &str, separator: &str) -> AppResult<FixMsg> {
    let state =  context.0.lock().unwrap();
    let AppState::Loaded { schema, .. } = &*state else {
        return Err("Error reading app state");
    };
    schema.from_string(input, separator)
}

#[derive(Deserialize, Serialize, Default)]
 struct MyConfig {
    schema_path: String,
}
//

fn load(app: &mut App) -> Result<(String, FixSchema)> {
    let schema_file = load_config(app)?;
    let schema = load_from_xml(&schema_file)?;
    Ok((schema_file, schema))
}

fn load_config(app: &mut App) -> Result<String> {
    let Some(file_buffer) = app.path_resolver().resolve_resource("config.json") else {
        return Err("No config file found")?;
    };
    let filename =  File::open(&file_buffer)?;
    let config: MyConfig = serde_json::from_reader(filename)?;
    Ok(config.schema_path)
}

fn load_from_xml(schema_file: &str) -> Result<FixSchema> {
    let reader = Reader::from_file(Path::new(schema_file))?;
    let schema = quick_xml::de::from_reader(reader.into_inner())?;
    Ok(schema)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {           
            match load(app) {
                Ok((file, schema)) => {
                    let app_state = AppState::Loaded { file_loaded: file, schema };
                    app.manage(Context(Mutex::new(app_state)));
                }
                Err(e) => {
                    let app_state = AppState::Unloaded { error: e.to_string() };
                    app.manage(Context(Mutex::new(app_state)));
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![read_fix, get_schema_file, set_schema_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 
