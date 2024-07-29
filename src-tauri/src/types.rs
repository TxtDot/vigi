use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::utils::write_tabs;

#[derive(Serialize, Deserialize, Debug)]
pub enum VigiError {
    Network,
    Parse,
    StateLock,
    StateUpdate,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VigiState {
    pub tabs_id_counter_path: PathBuf,
    pub current_tab_index_path: PathBuf,
    pub local_tabs_path: PathBuf,
    pub favorites_tabs_path: PathBuf,

    pub cache_dir: PathBuf,

    pub tabs_id_counter: usize,
    pub current_tab_index: usize,
    pub tabs: Vec<Tab>,
    pub favorites_tabs: Vec<Tab>,
}

impl VigiState {
    pub fn null() -> Self {
        Self {
            tabs_id_counter_path: PathBuf::new(),
            current_tab_index_path: PathBuf::new(),
            local_tabs_path: PathBuf::new(),
            favorites_tabs_path: PathBuf::new(),
            cache_dir: PathBuf::new(),

            tabs_id_counter: 0,
            current_tab_index: 0,
            tabs: Vec::new(),
            favorites_tabs: Vec::new(),
        }
    }

    pub fn update_current_tab_index(&mut self, new_index: usize) -> Result<(), VigiError> {
        self.current_tab_index = new_index;

        self.write_current_tab_index()?;
        Ok(())
    }

    fn write_current_tab_index(&mut self) -> Result<(), VigiError> {
        fs::write(
            &self.current_tab_index_path,
            self.current_tab_index.to_string(),
        )
        .map_err(|_| VigiError::StateUpdate)
    }

    fn write_id_counter(&mut self) -> Result<(), VigiError> {
        fs::write(&self.tabs_id_counter_path, self.tabs_id_counter.to_string())
            .map_err(|_| VigiError::StateUpdate)
    }

    pub fn update_tab(
        &mut self,
        tab_type: TabType,
        tab_title: String,
        tab_url: String,
    ) -> Result<(), VigiError> {
        match self.tabs.get_mut(self.current_tab_index) {
            Some(tab) => {
                *tab = Tab::new(tab_type, tab_title, tab_url, tab.id);

                write_tabs(&self.local_tabs_path, &self.tabs)?;

                Ok(())
            }
            None => Err(VigiError::StateUpdate),
        }
    }

    pub fn add_tab(&mut self) -> Result<(), VigiError> {
        self.tabs_id_counter += 1;
        self.tabs.push(Tab::new(
            TabType::HomePage,
            "New tab".to_string(),
            "".to_string(),
            self.tabs_id_counter,
        ));

        self.write_id_counter()?;
        write_tabs(&self.local_tabs_path, &self.tabs)?;

        self.current_tab_index = self.tabs.len() - 1;
        self.write_current_tab_index()?;

        Ok(())
    }

    pub fn remove_tab(&mut self, index: usize) -> Result<(), VigiError> {
        if self.tabs.len() - 1 == index && self.current_tab_index == index {
            if self.current_tab_index > 0 {
                self.current_tab_index -= 1;

                self.write_current_tab_index()?;
            }
        }

        self.tabs.remove(index);
        write_tabs(&self.local_tabs_path, &self.tabs)?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tab {
    ty: TabType,
    title: String,
    url: String,
    id: usize,
}

impl Tab {
    pub fn new(ty: TabType, title: String, url: String, id: usize) -> Self {
        Self { ty, title, url, id }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TabType {
    HomePage,
    Text,
}
