// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs::{self},
    sync::Mutex,
};

use dalet::{Argument, Body, Tag};

mod types;
mod utils;

use tauri::Manager;
use types::{TabType, VigiError, VigiState};
use utils::{read_or_create_jsonl, read_or_create_number};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn process_input(
    input: String,
    state: tauri::State<'_, Mutex<VigiState>>,
) -> Result<Vec<Tag>, VigiError> {
    // TODO: Implement mime type, language, protocol or search detection
    // TODO: Implement text links parsing

    println!("Processing: {}", input);

    match reqwest::get(input.clone()).await {
        Ok(res) => match res.text().await {
            Ok(res) => {
                update_tab(state, TabType::Text, res.clone(), input.clone())?;
                Ok(vec![Tag::new(0, Body::Text(res), Argument::Null)])
            }
            Err(_) => Err(VigiError::Parse),
        },
        Err(_) => Err(VigiError::Network),
    }
}

#[tauri::command]
fn get_state(state: tauri::State<Mutex<VigiState>>) -> VigiState {
    (*state.lock().unwrap()).clone()
}

#[tauri::command]
fn select_tab(state: tauri::State<Mutex<VigiState>>, index: usize) -> Result<(), VigiError> {
    match state.lock() {
        Ok(mut state) => {
            state.update_current_tab_index(index)?;
            Ok(())
        }
        Err(_) => Err(VigiError::StateLock),
    }
}

#[tauri::command]
fn add_tab(state: tauri::State<Mutex<VigiState>>) -> Result<(), VigiError> {
    match state.lock() {
        Ok(mut state) => {
            state.add_tab()?;
            Ok(())
        }
        Err(_) => Err(VigiError::StateLock),
    }
}

#[tauri::command]
fn remove_tab(state: tauri::State<Mutex<VigiState>>, index: usize) -> Result<(), VigiError> {
    match state.lock() {
        Ok(mut state) => {
            state.remove_tab(index)?;
            Ok(())
        }
        Err(_) => Err(VigiError::StateLock),
    }
}

fn update_tab(
    state: tauri::State<Mutex<VigiState>>,
    tab_type: TabType,
    tab_title: String,
    tab_url: String,
) -> Result<(), VigiError> {
    match state.lock() {
        Ok(mut state) => {
            state.update_tab(tab_type, tab_title, tab_url)?;
            Ok(())
        }
        Err(_) => Err(VigiError::StateLock),
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(VigiState::null()))
        .setup(setup_handler)
        .invoke_handler(tauri::generate_handler![
            process_input,
            get_state,
            select_tab,
            add_tab,
            remove_tab
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_handler(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error + 'static>> {
    println!("---Setup---");

    let app_handle = app.handle();

    let state = app.state::<Mutex<VigiState>>();
    let mut state = state.lock().unwrap();

    let config_dir = app_handle
        .path_resolver()
        .app_config_dir()
        .unwrap_or(std::path::PathBuf::new().join("config"));

    let local_data_dir = app_handle
        .path_resolver()
        .app_local_data_dir()
        .unwrap_or(std::path::PathBuf::new().join("data"));

    state.cache_dir = app_handle
        .path_resolver()
        .app_cache_dir()
        .unwrap_or(std::path::PathBuf::new().join("cache"));

    println!("Config dir: {}", config_dir.to_string_lossy());
    println!("Local data dir: {}", local_data_dir.to_string_lossy());
    println!("Cache dir: {}", state.cache_dir.to_string_lossy());

    println!("Checking config dir");

    // check if config/favorites.jsonl exists
    if !config_dir.exists() {
        println!("  Creating config dir");
        fs::create_dir_all(&config_dir)?;
    }

    state.favorites_tabs_path = config_dir.join("favorites.jsonl");
    state.favorites_tabs = read_or_create_jsonl(&state.favorites_tabs_path);

    println!("Checking local data dir");
    if !local_data_dir.exists() {
        println!("  Creating local data dir");
        fs::create_dir_all(&local_data_dir)?;
    }

    state.local_tabs_path = local_data_dir.join("tabs.jsonl");
    state.tabs = read_or_create_jsonl(&state.local_tabs_path);

    state.tabs_id_counter_path = local_data_dir.join("tabs_id_counter");
    state.tabs_id_counter = read_or_create_number(&state.tabs_id_counter_path);

    state.current_tab_index_path = local_data_dir.join("current_tab_index");
    state.current_tab_index = read_or_create_number(&state.current_tab_index_path);

    println!("---Setup done---");

    Ok(())
}
