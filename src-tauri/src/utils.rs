use std::{
    fs::{self, File},
    path::PathBuf,
};

use crate::types::{Tab, VigiError};

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

pub fn read_or_create_number(path: &PathBuf) -> usize {
    println!("  Getting number from {}", path.to_string_lossy());
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

pub fn create_file(path: PathBuf) -> PathBuf {
    if !path.exists() {
        println!("    Creating {}", path.to_string_lossy());
        File::create(&path).unwrap();
    }

    path
}

pub fn write_tabs(path: &PathBuf, tabs: &Vec<Tab>) -> Result<(), VigiError> {
    fs::write(
        path,
        tabs.iter()
            .map(|tab| serde_json::to_string(tab).unwrap())
            .collect::<Vec<String>>()
            .join("\n"),
    )
    .map_err(|_| VigiError::StateUpdate)
}
