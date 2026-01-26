use dalet::types::Tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum VigiError {
    ReadStateFailed,
    StateSaveFailed,
    NoPathToSave,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TabType {
    RENDER,
    BROWSER,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VigiState {
    pub current_tab: usize,
    pub tab_counter: usize,
    pub tabs: Vec<SiteTab>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteTab {
    pub id: usize,
    pub current_link: usize,
    pub links: Vec<TabLink>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TabLink {
    pub title: Option<String>,
    pub body: Option<Vec<Tag>>,

    pub ty: TabType,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermanentState {
    pub current_tab: usize,
    pub tabs: Vec<PermanentSiteTab>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermanentSiteTab {
    pub current_link: usize,
    pub links: Vec<PermanentTabLink>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PermanentTabLink {
    pub title: Option<String>,
    pub ty: TabType,
    pub uri: String,
}

// History

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub uri: String,
    pub title: Option<String>,
    pub timestamp: i64,
    pub visit_count: u32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct History {
    pub entries: Vec<HistoryEntry>,
}

// Bookmarks

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub uri: String,
    pub title: String,
    pub tags: Vec<String>,
    pub created_at: i64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Bookmarks {
    pub items: Vec<Bookmark>,
}

// Settings

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub theme: String,
    pub custom_css: Option<String>,
    pub search_engine: String,
    pub home_page: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: "green".to_string(),
            custom_css: None,
            search_engine: "https://s.dc09.xyz/search?q=%s".to_string(),
            home_page: "browser://main".to_string(),
        }
    }
}
