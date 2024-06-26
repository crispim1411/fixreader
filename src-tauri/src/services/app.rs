use std::sync::Mutex;
use tauri::{State, Manager};

use crate::models::*;
use crate::services::FixConverter;

struct Context(Mutex<AppState>);

enum AppState {
    Loaded { converter: FixConverter },
    Unloaded { error: AppError },
}

#[tauri::command]
fn get_schema_file(state: State<Context>) -> Result<String, AppError> {
    let Ok(state) = state.0.lock() else {
        return Err("Error reading app state".into());
    };
    match &*state {
        AppState::Loaded { converter } => {
            return Ok(converter.filename.clone());
        }
        AppState::Unloaded { error } => {
            return Err(error.clone());
        }
    }
}

#[tauri::command]
fn set_schema_file(context: State<Context>, path: &str)  -> Result<(), AppError> {
    let Ok(mut state) = context.0.lock() else {
        return Err("Error reading app state".into());
    };

    let mut converter = FixConverter::new();
    converter.load_from(path)?;

    *state = AppState::Loaded { converter };
    Ok(())
}

#[tauri::command]
fn read_fix(context: State<Context>, input: &str) -> Result<FixMessage, AppError> {
    let Ok(state) = context.0.lock() else {
        return Err("Error reading app state".into());
    };
    match &*state {
        AppState::Loaded { converter } => {
            return converter.from_str(input);
        }
        AppState::Unloaded { error } => {
            return Err(error.clone());
        }
    }
}


pub struct App;

impl App {
    pub fn start() {
        tauri::Builder::default()
            .setup(|app| {  
                let mut converter = FixConverter::new();    
                match converter.try_load() {
                    Ok(_) => {
                        let app_state = AppState::Loaded { converter };
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
}