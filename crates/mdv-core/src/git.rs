use std::path::Path;

use thiserror::Error;

use crate::diff::{line_diff, HunkSummary};

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

pub fn diff_against_head(file: &Path) -> Result<Vec<HunkSummary>, GitError> {
    let repo = git2::Repository::discover(file).map_err(|_| GitError::NotARepo)?;
    let workdir = repo.workdir().ok_or(GitError::NotARepo)?;
    let rel = file
        .strip_prefix(workdir)
        .map_err(|_| GitError::OutsideRepo)?;

    let head_text = read_head_blob(&repo, rel)?;
    let current_text = std::fs::read_to_string(file)?;
    Ok(line_diff(&head_text, &current_text))
}

fn read_head_blob(repo: &git2::Repository, rel: &Path) -> Result<String, GitError> {
    let head = repo.head()?.peel_to_tree()?;
    let entry = match head.get_path(rel) {
        Ok(e) => e,
        Err(_) => return Ok(String::new()),
    };
    let blob = repo.find_blob(entry.id())?;
    std::str::from_utf8(blob.content())
        .map(|s| s.to_string())
        .map_err(|_| GitError::NotUtf8)
}
