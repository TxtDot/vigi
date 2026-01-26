mod types;

use std::fs;

use dalet::types::Page;

use drova_plugins::requester_plugins;
use drova_sdk::requester::{Error, Requester, RequesterBuilder};
use tauri::Manager;
use std::time::{SystemTime, UNIX_EPOCH};
use types::{
    Bookmark, Bookmarks, History, HistoryEntry, PermanentSiteTab, PermanentState, PermanentTabLink,
    Settings, SiteTab, TabLink, VigiError, VigiState,
};

struct AppData<'a> {
    drova_core: Requester<'a>,
}

#[tauri::command]
async fn process_url(input: &str, app: tauri::AppHandle) -> Result<Page, Error> {
    app.state::<AppData>().drova_core.process(input).await
}

#[tauri::command]
async fn save_state(state: VigiState, app_handle: tauri::AppHandle) -> Result<(), VigiError> {
    let path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|_| VigiError::NoPathToSave)?
        .join("state.vigi");

    println!("saving state to: {}", path.to_str().unwrap());

    let permanent = PermanentState {
        current_tab: state.current_tab,
        tabs: state
            .tabs
            .into_iter()
            .map(
                |SiteTab {
                     id: _,
                     current_link,
                     links,
                 }| {
                    return PermanentSiteTab {
                        current_link,
                        links: links
                            .into_iter()
                            .map(
                                |TabLink {
                                     ty,
                                     uri,
                                     title,
                                     body: _,
                                 }| {
                                    return PermanentTabLink { ty, uri, title };
                                },
                            )
                            .collect(),
                    };
                },
            )
            .collect(),
    };

    Ok(fs::write(
        path,
        bitcode::serialize(&permanent).map_err(|_| VigiError::StateSaveFailed)?,
    )
    .map_err(|_| VigiError::StateSaveFailed)?)
}

#[tauri::command]
async fn get_state(app_handle: tauri::AppHandle) -> Result<VigiState, VigiError> {
    let path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|_| VigiError::NoPathToSave)?
        .join("state.vigi");

    println!("getting state from: {}", path.to_str().unwrap());

    let permanent: PermanentState =
        bitcode::deserialize(&fs::read(path).map_err(|_| VigiError::ReadStateFailed)?)
            .map_err(|_| VigiError::ReadStateFailed)?;

    let mut counter = 0;

    Ok(VigiState {
        current_tab: permanent.current_tab,
        tab_counter: permanent.tabs.len(),
        tabs: permanent
            .tabs
            .into_iter()
            .map(
                |PermanentSiteTab {
                     current_link,
                     links,
                 }| {
                    counter += 1;

                    SiteTab {
                        id: counter - 1,
                        current_link,
                        links: links
                            .into_iter()
                            .map(|PermanentTabLink { title, ty, uri }| TabLink {
                                ty,
                                title,
                                body: None,
                                uri,
                            })
                            .collect(),
                    }
                },
            )
            .collect(),
    })
}

// Helper function to get current timestamp
fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

// History commands

#[tauri::command]
async fn get_history(
    limit: Option<u32>,
    app_handle: tauri::AppHandle,
) -> Result<Vec<HistoryEntry>, VigiError> {
    let path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|_| VigiError::NoPathToSave)?
        .join("history.vigi");

    let history: History = match fs::read(&path) {
        Ok(data) => bitcode::deserialize(&data).unwrap_or_default(),
        Err(_) => History::default(),
    };

    let mut entries = history.entries;
    entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    if let Some(limit) = limit {
        entries.truncate(limit as usize);
    }

    Ok(entries)
}

#[tauri::command]
async fn add_history(
    uri: String,
    title: Option<String>,
    app_handle: tauri::AppHandle,
) -> Result<(), VigiError> {
    let path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|_| VigiError::NoPathToSave)?
        .join("history.vigi");

    let mut history: History = match fs::read(&path) {
        Ok(data) => bitcode::deserialize(&data).unwrap_or_default(),
        Err(_) => History::default(),
    };

    // Check if URI already exists and update visit count
    if let Some(entry) = history.entries.iter_mut().find(|e| e.uri == uri) {
        entry.visit_count += 1;
        entry.timestamp = now();
        if title.is_some() {
            entry.title = title;
        }
    } else {
        history.entries.push(HistoryEntry {
            uri,
            title,
            timestamp: now(),
            visit_count: 1,
        });
    }

    fs::write(
        path,
        bitcode::serialize(&history).map_err(|_| VigiError::StateSaveFailed)?,
    )
    .map_err(|_| VigiError::StateSaveFailed)?;

    Ok(())
}

#[tauri::command]
async fn get_frequent_sites(
    limit: u32,
    app_handle: tauri::AppHandle,
) -> Result<Vec<HistoryEntry>, VigiError> {
    let path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|_| VigiError::NoPathToSave)?
        .join("history.vigi");

    let history: History = match fs::read(&path) {
        Ok(data) => bitcode::deserialize(&data).unwrap_or_default(),
        Err(_) => History::default(),
    };

    let mut entries = history.entries;
    entries.sort_by(|a, b| b.visit_count.cmp(&a.visit_count));
    entries.truncate(limit as usize);

    Ok(entries)
}

#[tauri::command]
async fn search_history(
    query: String,
    app_handle: tauri::AppHandle,
) -> Result<Vec<HistoryEntry>, VigiError> {
    let path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|_| VigiError::NoPathToSave)?
        .join("history.vigi");

    let history: History = match fs::read(&path) {
        Ok(data) => bitcode::deserialize(&data).unwrap_or_default(),
        Err(_) => History::default(),
    };

    let query_lower = query.to_lowercase();
    let entries: Vec<HistoryEntry> = history
        .entries
        .into_iter()
        .filter(|e| {
            e.uri.to_lowercase().contains(&query_lower)
                || e.title
                    .as_ref()
                    .map(|t| t.to_lowercase().contains(&query_lower))
                    .unwrap_or(false)
        })
        .collect();

    Ok(entries)
}

#[tauri::command]
async fn clear_history(app_handle: tauri::AppHandle) -> Result<(), VigiError> {
    let path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|_| VigiError::NoPathToSave)?
        .join("history.vigi");

    let _ = fs::remove_file(path);
    Ok(())
}

// Bookmarks commands

#[tauri::command]
async fn get_bookmarks(app_handle: tauri::AppHandle) -> Result<Vec<Bookmark>, VigiError> {
    let path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|_| VigiError::NoPathToSave)?
        .join("bookmarks.vigi");

    let bookmarks: Bookmarks = match fs::read(&path) {
        Ok(data) => bitcode::deserialize(&data).unwrap_or_default(),
        Err(_) => Bookmarks::default(),
    };

    Ok(bookmarks.items)
}

#[tauri::command]
async fn add_bookmark(
    uri: String,
    title: String,
    tags: Vec<String>,
    app_handle: tauri::AppHandle,
) -> Result<(), VigiError> {
    let path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|_| VigiError::NoPathToSave)?
        .join("bookmarks.vigi");

    let mut bookmarks: Bookmarks = match fs::read(&path) {
        Ok(data) => bitcode::deserialize(&data).unwrap_or_default(),
        Err(_) => Bookmarks::default(),
    };

    // Remove existing bookmark with same URI if exists
    bookmarks.items.retain(|b| b.uri != uri);

    bookmarks.items.push(Bookmark {
        uri,
        title,
        tags,
        created_at: now(),
    });

    fs::write(
        path,
        bitcode::serialize(&bookmarks).map_err(|_| VigiError::StateSaveFailed)?,
    )
    .map_err(|_| VigiError::StateSaveFailed)?;

    Ok(())
}

#[tauri::command]
async fn remove_bookmark(uri: String, app_handle: tauri::AppHandle) -> Result<(), VigiError> {
    let path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|_| VigiError::NoPathToSave)?
        .join("bookmarks.vigi");

    let mut bookmarks: Bookmarks = match fs::read(&path) {
        Ok(data) => bitcode::deserialize(&data).unwrap_or_default(),
        Err(_) => Bookmarks::default(),
    };

    bookmarks.items.retain(|b| b.uri != uri);

    fs::write(
        path,
        bitcode::serialize(&bookmarks).map_err(|_| VigiError::StateSaveFailed)?,
    )
    .map_err(|_| VigiError::StateSaveFailed)?;

    Ok(())
}

#[tauri::command]
async fn is_bookmarked(uri: String, app_handle: tauri::AppHandle) -> Result<bool, VigiError> {
    let path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|_| VigiError::NoPathToSave)?
        .join("bookmarks.vigi");

    let bookmarks: Bookmarks = match fs::read(&path) {
        Ok(data) => bitcode::deserialize(&data).unwrap_or_default(),
        Err(_) => Bookmarks::default(),
    };

    Ok(bookmarks.items.iter().any(|b| b.uri == uri))
}

#[tauri::command]
async fn get_all_tags(app_handle: tauri::AppHandle) -> Result<Vec<String>, VigiError> {
    let path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|_| VigiError::NoPathToSave)?
        .join("bookmarks.vigi");

    let bookmarks: Bookmarks = match fs::read(&path) {
        Ok(data) => bitcode::deserialize(&data).unwrap_or_default(),
        Err(_) => Bookmarks::default(),
    };

    let mut tags: Vec<String> = bookmarks
        .items
        .into_iter()
        .flat_map(|b| b.tags)
        .collect();

    tags.sort();
    tags.dedup();

    Ok(tags)
}

// Settings commands

#[tauri::command]
async fn get_settings(app_handle: tauri::AppHandle) -> Result<Settings, VigiError> {
    let path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|_| VigiError::NoPathToSave)?
        .join("settings.vigi");

    let settings: Settings = match fs::read(&path) {
        Ok(data) => bitcode::deserialize(&data).unwrap_or_default(),
        Err(_) => Settings::default(),
    };

    Ok(settings)
}

#[tauri::command]
async fn save_settings(settings: Settings, app_handle: tauri::AppHandle) -> Result<(), VigiError> {
    let path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|_| VigiError::NoPathToSave)?
        .join("settings.vigi");

    fs::write(
        path,
        bitcode::serialize(&settings).map_err(|_| VigiError::StateSaveFailed)?,
    )
    .map_err(|_| VigiError::StateSaveFailed)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(AppData {
                drova_core: RequesterBuilder::default()
                    .plugin(requester_plugins)
                    .build(),
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            process_url,
            save_state,
            get_state,
            // History
            get_history,
            add_history,
            get_frequent_sites,
            search_history,
            clear_history,
            // Bookmarks
            get_bookmarks,
            add_bookmark,
            remove_bookmark,
            is_bookmarked,
            get_all_tags,
            // Settings
            get_settings,
            save_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
