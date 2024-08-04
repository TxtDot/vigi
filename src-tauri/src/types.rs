use dalet::daletl::{Tag, ToDaletlPage};
use dalet::typed::Tag::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

use crate::utils::write_tabs;

use crate::process_input::process_input;

#[derive(Serialize, Deserialize, Debug)]
pub enum VigiError {
    Network,
    Parse,
    StateLock,
    StateUpdate,
    Filesystem,
    Config,
    GetTab,

    UnsupportedProtocol,
    UnsupportedMimeType,
    InvalidMimeType,

    InvalidCharset,
}

#[derive(Debug, Clone)]
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
    pub cached_inputs: HashMap<String, VigiOutput>,
}

#[derive(Debug, Clone)]
pub struct VigiOutput {
    pub title: String,
    pub data: Vec<Tag>,
}

impl VigiOutput {
    pub fn new(title: String, data: Vec<Tag>) -> Self {
        Self { title, data }
    }
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
            cached_inputs: HashMap::new(),
        }
    }

    pub fn select_tab(&mut self, new_index: usize) -> Result<(), VigiError> {
        self.update_current_tab_index(new_index)?;
        self.update_top_bar_input()?;
        Ok(())
    }

    fn update_current_tab_index(&mut self, new_index: usize) -> Result<(), VigiError> {
        self.current_tab_index = new_index;

        fs::write(
            &self.current_tab_index_path,
            self.current_tab_index.to_string(),
        )
        .map_err(|_| VigiError::StateUpdate)?;

        Ok(())
    }

    pub fn update_top_bar_input(&mut self) -> Result<(), VigiError> {
        self.top_bar_input = self
            .tabs
            .get(self.current_tab_index)
            .ok_or(VigiError::StateUpdate)?
            .url
            .clone();

        Ok(())
    }

    fn write_id_counter(&mut self) -> Result<(), VigiError> {
        fs::write(&self.tabs_id_counter_path, self.tabs_id_counter.to_string())
            .map_err(|_| VigiError::StateUpdate)
    }

    async fn process_input(&mut self, ignore_cache: bool) -> Result<(), VigiError> {
        println!("process_input \"{}\"", self.top_bar_input);

        let cached = !ignore_cache && self.cached_inputs.contains_key(&self.top_bar_input);

        let output = {
            if cached {
                self.cached_inputs.get(&self.top_bar_input).unwrap().clone()
            } else if self.top_bar_input.is_empty() {
                VigiOutput::new(
                    "Homepage".to_owned(),
                    vec![El("Type something in the address bar".into())].to_dl_page(),
                )
            } else {
                process_input(&self.top_bar_input).await?
            }
        };

        self.update_tab(output.title.clone(), self.top_bar_input.clone())?;
        self.current_data = output.data.clone();

        if !cached {
            self.cached_inputs
                .insert(self.top_bar_input.clone(), output);
        }

        Ok(())
    }

    pub fn update_input(&mut self, input: String) {
        self.top_bar_input = input;
    }

    pub async fn load_input_force(&mut self) -> Result<(), VigiError> {
        self.process_input(true).await
    }

    pub async fn load_input(&mut self) -> Result<(), VigiError> {
        self.process_input(false).await
    }

    fn update_tab(&mut self, tab_title: String, tab_url: String) -> Result<(), VigiError> {
        match self.tabs.get_mut(self.current_tab_index) {
            Some(tab) => {
                *tab = Tab::new(tab_title, tab_url, tab.id);

                write_tabs(&self.local_tabs_path, &self.tabs)?;

                Ok(())
            }
            None => Err(VigiError::StateUpdate),
        }
    }

    pub async fn add_tab(&mut self) -> Result<(), VigiError> {
        self.tabs_id_counter += 1;
        self.tabs.push(Tab::new(
            "Home".to_string(),
            "".to_string(),
            self.tabs_id_counter,
        ));

        self.write_id_counter()?;
        write_tabs(&self.local_tabs_path, &self.tabs)?;

        self.select_tab(self.tabs.len() - 1)?;
        self.load_input().await?;

        Ok(())
    }

    pub fn remove_tab(&mut self, index: usize) -> Result<(), VigiError> {
        if self.current_tab_index >= index {
            if self.current_tab_index > 0 {
                self.select_tab(self.current_tab_index - 1)?;
            }
        }

        self.cached_inputs
            .remove(&self.tabs.get(index).ok_or(VigiError::GetTab)?.url);

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
    pub title: String,
    pub url: String,
    pub id: usize,
}

impl Tab {
    pub fn new(title: String, url: String, id: usize) -> Self {
        Self { title, url, id }
    }
}
