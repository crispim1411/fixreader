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
    if let AppState::Loaded { converter } = &*state {
        return Ok(converter.filename.clone());
    }
    return Err("No file".into());
}

#[tauri::command]
fn set_schema_file(context: State<Context>, path: &str)  -> Result<(), AppError> {
    let mut state = context.0.lock().unwrap();

    let mut converter = FixConverter::new();
    converter.load_from(path)?;

    *state = AppState::Loaded { converter };
    Ok(())
}

//
#[tauri::command]
fn read_fix(context: State<Context>, input: &str, separator: &str) -> Result<FixMessage, AppError> {
    let state =  context.0.lock().unwrap();
    if let AppState::Loaded { converter } = &*state {
        return converter.from_str(input);
    };
    return Err("Error reading app state".into());
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