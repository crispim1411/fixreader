// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::services::App;

mod models;
pub mod services;

fn main() {
    App::start();
} 
