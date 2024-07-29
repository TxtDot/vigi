use std::{
    fs::{self, File},
    path::PathBuf,
};

use crate::types::Tab;

pub fn read_jsonl_tabs(path: &PathBuf) -> Vec<Tab> {
    fs::read_to_string(&path)
        .unwrap_or_default()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

pub fn read_or_create_jsonl(path: &PathBuf) -> Vec<Tab> {
    println!("  Getting tabs from {}", path.to_string_lossy());
    if path.exists() {
        read_jsonl_tabs(path)
    } else {
        println!("    Creating {}", path.to_string_lossy());
        File::create(path).unwrap();
        Vec::new()
    }
}

pub fn read_or_create_current_tab_index(path: &PathBuf) -> usize {
    println!(
        "  Getting current tab index from {}",
        path.to_string_lossy()
    );
    if path.exists() {
        fs::read_to_string(path)
            .unwrap()
            .parse::<usize>()
            .unwrap_or(0)
    } else {
        println!("    Creating {}", path.to_string_lossy());
        fs::write(path, "0").unwrap();
        0
    }
}
