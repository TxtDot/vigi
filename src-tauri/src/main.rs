// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{self};

mod process_input;
mod types;
mod utils;

use tokio::sync::Mutex;
use types::{VigiError, VigiJsState, VigiState};
use utils::{read_or_create_jsonl, read_or_create_number};

#[tauri::command]
async fn update_input(
    input: String,
    new_tab: bool,
    state: tauri::State<'_, Mutex<VigiState>>,
) -> Result<(), VigiError> {
    state.lock().await.update_input(input, new_tab).await?;

    Ok(())
}

#[tauri::command]
async fn load_input(state: tauri::State<'_, Mutex<VigiState>>) -> Result<(), VigiError> {
    state.lock().await.load_input().await?;

    Ok(())
}

#[tauri::command]
async fn load_input_force(state: tauri::State<'_, Mutex<VigiState>>) -> Result<(), VigiError> {
    state.lock().await.load_input_force().await?;

    Ok(())
}

#[tauri::command]
async fn get_js_state(state: tauri::State<'_, Mutex<VigiState>>) -> Result<VigiJsState, VigiError> {
    Ok(state.lock().await.get_js_state())
}

#[tauri::command]
async fn select_tab(
    index: usize,
    state: tauri::State<'_, Mutex<VigiState>>,
) -> Result<(), VigiError> {
    state.lock().await.select_tab(index)
}

#[tauri::command]
async fn add_tab(state: tauri::State<'_, Mutex<VigiState>>) -> Result<(), VigiError> {
    state.lock().await.add_tab().await
}

#[tauri::command]
async fn remove_tab(
    state: tauri::State<'_, Mutex<VigiState>>,
    index: usize,
) -> Result<(), VigiError> {
    state.lock().await.remove_tab(index)
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(VigiState::null()))
        .invoke_handler(tauri::generate_handler![
            update_input,
            get_js_state,
            select_tab,
            add_tab,
            remove_tab,
            setup,
            load_input,
            load_input_force
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn setup(
    state: tauri::State<'_, Mutex<VigiState>>,
    app_handle: tauri::AppHandle,
) -> Result<(), VigiError> {
    println!("---Setup---");

    let mut state = state.lock().await;

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
        fs::create_dir_all(&config_dir).map_err(|_| VigiError::Config)?;
    }

    state.favorites_tabs_path = config_dir.join("favorites.jsonl");
    state.favorites_tabs = read_or_create_jsonl(&state.favorites_tabs_path);

    println!("Checking local data dir");
    if !local_data_dir.exists() {
        println!("  Creating local data dir");
        fs::create_dir_all(&local_data_dir).map_err(|_| VigiError::Config)?;
    }

    state.local_tabs_path = local_data_dir.join("tabs.jsonl");
    state.tabs = read_or_create_jsonl(&state.local_tabs_path);

    state.tabs_id_counter_path = local_data_dir.join("tabs_id_counter");
    state.tabs_id_counter = read_or_create_number(&state.tabs_id_counter_path);

    state.current_tab_index_path = local_data_dir.join("current_tab_index");
    state.current_tab_index = read_or_create_number(&state.current_tab_index_path);

    state.update_top_bar_input()?;

    println!("---Setup done---");

    Ok(())
}
