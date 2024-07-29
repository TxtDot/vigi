// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dalet::{Argument, Body, Tag};

mod types;
use types::VigiError;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn process_input(input: String) -> Result<Vec<Tag>, VigiError> {
    // TODO: Implement mime type, language, protocol or search detection
    // TODO: Implement text links parsing

    match reqwest::get(input).await {
        Ok(res) => match res.text().await {
            Ok(res) => Ok(vec![Tag::new(0, Body::Text(res), Argument::Null)]),
            Err(_) => Err(VigiError::ParseError),
        },
        Err(_) => Err(VigiError::NetworkError),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process_input])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
