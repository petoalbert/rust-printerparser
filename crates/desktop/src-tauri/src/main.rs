// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod serde_instances;

use crate::serde_instances::ShortCommitRecordWrapper;
use parserprinter::api::{
    get_current_brach::get_current_brach, list_branches_command::list_braches,
    log_checkpoints_command::log_checkpoints,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn db_list_braches(db_path: &str) -> Vec<String> {
    list_braches(db_path)
}

#[tauri::command]
fn db_log_checkpoints(db_path: &str) -> Vec<ShortCommitRecordWrapper> {
    log_checkpoints(db_path, None)
        .into_iter()
        .map(ShortCommitRecordWrapper)
        .collect()
}

#[tauri::command]
fn db_get_current_branch(db_path: &str) -> String {
    get_current_brach(db_path)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            db_list_braches,
            db_log_checkpoints,
            db_get_current_branch
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
