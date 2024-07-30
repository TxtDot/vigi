use dalet::{Argument, Body, Tag};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::utils::write_tabs;

#[derive(Serialize, Deserialize, Debug)]
pub enum VigiError {
    Network,
    Parse,
    StateLock,
    StateUpdate,
    Filesystem,
    Config,
}

#[derive(Serialize, Debug, Clone)]
pub struct VigiState {
    pub tabs_id_counter_path: PathBuf,
    pub current_tab_index_path: PathBuf,
    pub local_tabs_path: PathBuf,
    pub favorites_tabs_path: PathBuf,

    pub cache_dir: PathBuf,

    // Persistent
    pub tabs_id_counter: usize,
    pub current_tab_index: usize,
    pub tabs: Vec<Tab>,
    pub favorites_tabs: Vec<Tab>,

    // Temporary
    pub top_bar_input: String,
    pub current_data: Vec<Tag>,
}

#[derive(Serialize, Debug, Clone)]
pub struct VigiJsState {
    pub current_tab_index: usize,
    pub tabs: Vec<Tab>,
    pub favorites_tabs: Vec<Tab>,

    pub top_bar_input: String,
    pub current_data: Vec<Tag>,
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

            top_bar_input: "".to_string(),
            current_data: Vec::new(),
        }
    }

    pub async fn select_tab(&mut self, new_index: usize) -> Result<(), VigiError> {
        self.current_tab_index = new_index;
        self.write_current_tab_index()?;

        self.update_top_bar_input();
        Ok(())
    }

    pub fn update_top_bar_input(&mut self) {
        self.top_bar_input = self.tabs[self.current_tab_index].url.clone();
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

    async fn process_input(&mut self) -> Result<(), VigiError> {
        // TODO: Implement mime type, language, protocol or search detection
        // TODO: Implement text links parsing

        println!("process_input {{\n  \"{}\"", self.top_bar_input);

        let result = match self.top_bar_input.as_str() {
            "" => {
                self.update_tab(TabType::HomePage, "Home".to_owned(), "".to_owned())?;

                self.current_data = vec![Tag::new(
                    0,
                    Body::Text("Type something in the address bar".to_owned()),
                    Argument::Null,
                )];

                Ok(())
            }
            input => match reqwest::get(input).await {
                Ok(res) => match res.text().await {
                    Ok(res) => {
                        let mut truncated = res.clone();
                        truncated.truncate(50);

                        self.update_tab(TabType::Text, truncated, input.to_owned())?;

                        self.current_data = vec![Tag::new(0, Body::Text(res), Argument::Null)];

                        Ok(())
                    }
                    Err(_) => Err(VigiError::Parse),
                },
                Err(_) => Err(VigiError::Network),
            },
        };

        if result.is_ok() {
            println!("  Ok\n}}");
        } else {
            println!("  Err\n}}");
        }

        result
    }

    pub async fn update_input(&mut self, input: String) -> Result<(), VigiError> {
        self.top_bar_input = input;
        self.process_input().await
    }

    pub async fn load_tab(&mut self) -> Result<(), VigiError> {
        self.process_input().await
    }

    fn update_tab(
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
            "Home".to_string(),
            "".to_string(),
            self.tabs_id_counter,
        ));

        self.write_id_counter()?;
        write_tabs(&self.local_tabs_path, &self.tabs)?;

        self.current_tab_index = self.tabs.len() - 1;
        self.write_current_tab_index()?;

        self.update_top_bar_input();

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

    pub fn get_js_state(&self) -> VigiJsState {
        VigiJsState {
            current_tab_index: self.current_tab_index,
            tabs: self.tabs.clone(),
            favorites_tabs: self.favorites_tabs.clone(),

            top_bar_input: self.top_bar_input.clone(),
            current_data: self.current_data.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tab {
    pub ty: TabType,
    pub title: String,
    pub url: String,
    pub id: usize,
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
