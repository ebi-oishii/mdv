use std::path::PathBuf;

use mdv_core::diff::{DiffLine, HunkSummary};

#[tauri::command]
pub async fn git_is_repo(path: PathBuf) -> Result<bool, String> {
    Ok(mdv_core::git::is_in_repo(&path))
}

#[tauri::command]
pub async fn git_hunks(path: PathBuf, current_text: String) -> Result<Vec<HunkSummary>, String> {
    mdv_core::git::diff_text_against_head(&path, &current_text).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn git_full_diff(
    path: PathBuf,
    current_text: String,
) -> Result<Vec<DiffLine>, String> {
    mdv_core::git::full_diff_against_head(&path, &current_text).map_err(|e| e.to_string())
}
