use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum VigiError {
    NetworkError,
    ParseError,
}

pub struct State {
    pub current_tab_index_path: PathBuf,
    pub local_tabs_path: PathBuf,
    pub favorites_tabs_path: PathBuf,

    pub cache_dir: PathBuf,

    pub current_tab_index: usize,
    pub tabs: Vec<Tab>,
    pub favorites_tabs: Vec<Tab>,
}

impl State {
    pub fn null() -> Self {
        Self {
            current_tab_index_path: PathBuf::new(),
            local_tabs_path: PathBuf::new(),
            favorites_tabs_path: PathBuf::new(),
            cache_dir: PathBuf::new(),
            current_tab_index: 0,
            tabs: Vec::new(),
            favorites_tabs: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tab {
    ty: TabType,
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TabType {
    HomePage,
    Text,
}
