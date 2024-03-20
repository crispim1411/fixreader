// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fixreader::{FixSchema, FixMsg, AppError, Cache};
use quick_xml::Reader;
use tauri::{App, Manager, State};
use std::{sync::Mutex, path::Path};

struct Context(Mutex<AppState>);

enum AppState {
    Loaded { 
        file_loaded: String, 
        schema: FixSchema, 
    },
    Unloaded { error: AppError },
}

#[tauri::command]
fn get_schema_file(state: State<Context>) -> Result<String, AppError> {
    let Ok(state) = state.0.lock() else {
        return Err("Error reading app state".into());
    };
    if let AppState::Loaded { file_loaded, .. } = &*state {
        return Ok(file_loaded.clone());
    }
    return Err("No file".into());
}

#[tauri::command]
fn set_schema_file(context: State<Context>, path: &str)  -> Result<(), AppError> {
    let mut state = context.0.lock().unwrap();
    let schema = read_from_xml(path)?;

    *state = AppState::Loaded { file_loaded: path.into(), schema };
    Cache::save(path)?;

    Ok(())
}

#[tauri::command]
fn read_fix(context: State<Context>, input: &str, separator: &str) -> Result<FixMsg, AppError> {
    let state =  context.0.lock().unwrap();
    let AppState::Loaded { schema, .. } = &*state else {
        return Err("Error reading app state".into());
    };
    schema.from_string(input, separator)
}

fn load() -> Result<(String, FixSchema), AppError> {
    let schema_file = Cache::load()?;
    let schema = read_from_xml(&schema_file)?;
    Ok((schema_file, schema))
}

fn read_from_xml(schema_file: &str) -> Result<FixSchema, AppError> {
    let reader = Reader::from_file(Path::new(schema_file))?;
    let schema = quick_xml::de::from_reader(reader.into_inner())?;
    Ok(schema)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {           
            match load() {
                Ok((file, schema)) => {
                    let app_state = AppState::Loaded { file_loaded: file, schema };
                    app.manage(Context(Mutex::new(app_state)));
                }
                Err(error) => {
                    let app_state = AppState::Unloaded { error };
                    app.manage(Context(Mutex::new(app_state)));
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![read_fix, get_schema_file, set_schema_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 
