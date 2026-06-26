use std::path::Path;

use thiserror::Error;

use crate::diff::{full_diff, line_diff, DiffLine, HunkSummary};

#[derive(Debug, Error)]
pub enum GitError {
    #[error("not in a git repository")]
    NotARepo,
    #[error("git operation failed: {0}")]
    Git(#[from] git2::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("file is outside the discovered repository")]
    OutsideRepo,
    #[error("blob content is not valid utf-8")]
    NotUtf8,
}

pub fn is_in_repo(file: &Path) -> bool {
    git2::Repository::discover(file).is_ok()
}

pub fn diff_against_head(file: &Path) -> Result<Vec<HunkSummary>, GitError> {
    let current = std::fs::read_to_string(file)?;
    diff_text_against_head(file, &current)
}

pub fn diff_text_against_head(file: &Path, current: &str) -> Result<Vec<HunkSummary>, GitError> {
    let head = head_text_for(file)?;
    Ok(line_diff(&head, current))
}

pub fn full_diff_against_head(file: &Path, current: &str) -> Result<Vec<DiffLine>, GitError> {
    let head = head_text_for(file)?;
    Ok(full_diff(&head, current))
}

fn head_text_for(file: &Path) -> Result<String, GitError> {
    let repo = git2::Repository::discover(file).map_err(|_| GitError::NotARepo)?;
    let workdir = repo.workdir().ok_or(GitError::NotARepo)?;
    let rel = file
        .strip_prefix(workdir)
        .map_err(|_| GitError::OutsideRepo)?;
    read_head_blob(&repo, rel)
}

fn read_head_blob(repo: &git2::Repository, rel: &Path) -> Result<String, GitError> {
    let head = match repo.head() {
        Ok(h) => h.peel_to_tree()?,
        Err(_) => return Ok(String::new()),
    };
    let entry = match head.get_path(rel) {
        Ok(e) => e,
        Err(_) => return Ok(String::new()),
    };
    let blob = repo.find_blob(entry.id())?;
    std::str::from_utf8(blob.content())
        .map(|s| s.to_string())
        .map_err(|_| GitError::NotUtf8)
}
