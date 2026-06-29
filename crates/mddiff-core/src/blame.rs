//! Per-line blame across Git history + local save snapshots.
//!
//! For each line of the current buffer, identify whether it was last touched
//! by a Git commit (we hand back sha / author / date / summary), by a local
//! save snapshot (author = "local", date = latest snapshot timestamp), or by
//! the live buffer only (no commit, no snapshot — "unsaved").
//!
//! Implementation uses git2's `blame_buffer`: it takes a HEAD blame and an
//! in-memory buffer, returning a new blame where lines that differ from
//! HEAD are attributed to a NULL commit. Those NULL-commit lines are then
//! relabelled as Local / Buffer depending on whether snapshots exist for
//! this file.
//!
//! Caveats:
//! - We don't (yet) attribute each "local" line to a SPECIFIC snapshot —
//!   v1 just stamps the latest snapshot's timestamp. Per-snapshot
//!   attribution would mean re-running diffs against each snapshot.
//! - The "base" is always HEAD. Blaming at an arbitrary revspec isn't
//!   wired through yet (DiffView's base picker doesn't affect blame).

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlameOrigin {
    /// Line content matches HEAD (modulo snapshot lineage) and Git has a
    /// real commit for it.
    Git,
    /// Line differs from HEAD but the file has saved snapshots. We
    /// optimistically attribute the line to the latest snapshot.
    Local,
    /// Line differs from HEAD and no snapshots exist (or no snapshot
    /// contains this content) — purely an in-buffer addition.
    Buffer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlameLine {
    /// 1-based line number into `current_text`.
    pub line_no: u32,
    pub origin: BlameOrigin,
    /// Full SHA when origin is Git, else None.
    pub sha: Option<String>,
    /// 8-char abbreviation when origin is Git, else None.
    pub short_sha: Option<String>,
    /// `"<name>"` for Git lines, literally `"local"` for Local / Buffer.
    pub author: String,
    pub email: Option<String>,
    /// Unix seconds. 0 when unknown (buffer, no snapshot).
    pub date_ts: i64,
    /// First line of the commit message, for Git lines.
    pub summary: Option<String>,
}

#[derive(Debug, Error)]
pub enum BlameError {
    #[error("not in a git repository")]
    NotARepo,
    #[error("git operation failed: {0}")]
    Git(#[from] git2::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("file is outside the discovered repository")]
    OutsideRepo,
}

/// `latest_snapshot_ts_ms` — unix-MILLISECONDS of the most recent local
/// snapshot for this file, if any. Lines that aren't in HEAD get attributed
/// to that timestamp (origin=Local). If `None`, those lines are marked as
/// Buffer (live in-buffer addition, never saved).
pub fn compute(
    file: &Path,
    current_text: &str,
    latest_snapshot_ts_ms: Option<i64>,
) -> Result<Vec<BlameLine>, BlameError> {
    let file_abs = canonicalize_lossy(file);
    let repo =
        git2::Repository::discover(&file_abs).map_err(|_| BlameError::NotARepo)?;
    let workdir = repo.workdir().ok_or(BlameError::NotARepo)?;
    let workdir_abs = canonicalize_lossy(workdir);
    let rel = file_abs
        .strip_prefix(&workdir_abs)
        .map_err(|_| BlameError::OutsideRepo)?;

    let line_count = count_lines(current_text);
    if line_count == 0 {
        return Ok(Vec::new());
    }

    // Empty repo / file not in HEAD yet → everything is local / buffer.
    let head_blame = match repo.blame_file(rel, None) {
        Ok(b) => b,
        Err(_) => return Ok(all_local(line_count, latest_snapshot_ts_ms)),
    };

    let buffer_blame = head_blame.blame_buffer(current_text.as_bytes())?;

    let mut result = Vec::with_capacity(line_count);
    for line_idx in 0..line_count {
        let line_no = (line_idx + 1) as u32;
        let hunk = buffer_blame.get_line(line_idx + 1);
        match hunk {
            Some(hunk) => {
                let oid = hunk.final_commit_id();
                if oid.is_zero() {
                    result.push(local_line(line_no, latest_snapshot_ts_ms));
                } else {
                    result.push(git_line(&repo, line_no, oid, &hunk));
                }
            }
            None => result.push(local_line(line_no, latest_snapshot_ts_ms)),
        }
    }

    Ok(result)
}

fn git_line(
    repo: &git2::Repository,
    line_no: u32,
    oid: git2::Oid,
    hunk: &git2::BlameHunk<'_>,
) -> BlameLine {
    let sig = hunk.final_signature();
    let author = sig.name().unwrap_or("").to_string();
    let email = sig.email().map(|s| s.to_string());
    let date_ts = sig.when().seconds();
    let sha = oid.to_string();
    let short_sha = sha.chars().take(8).collect::<String>();
    let summary = repo
        .find_commit(oid)
        .ok()
        .and_then(|c| c.summary().map(|s| s.to_string()));
    BlameLine {
        line_no,
        origin: BlameOrigin::Git,
        sha: Some(sha),
        short_sha: Some(short_sha),
        author,
        email,
        date_ts,
        summary,
    }
}

fn local_line(line_no: u32, latest_snapshot_ts_ms: Option<i64>) -> BlameLine {
    BlameLine {
        line_no,
        origin: if latest_snapshot_ts_ms.is_some() {
            BlameOrigin::Local
        } else {
            BlameOrigin::Buffer
        },
        sha: None,
        short_sha: None,
        author: "local".to_string(),
        email: None,
        date_ts: latest_snapshot_ts_ms.map(|ms| ms / 1000).unwrap_or(0),
        summary: None,
    }
}

fn all_local(count: usize, latest_snapshot_ts_ms: Option<i64>) -> Vec<BlameLine> {
    (1..=count)
        .map(|n| local_line(n as u32, latest_snapshot_ts_ms))
        .collect()
}

/// Git's blame numbering matches what `str::lines()` produces — the trailing
/// newline doesn't add a phantom line. Empty text → 0 lines.
fn count_lines(text: &str) -> usize {
    if text.is_empty() {
        0
    } else {
        text.lines().count()
    }
}

fn canonicalize_lossy(p: &Path) -> PathBuf {
    p.canonicalize().unwrap_or_else(|_| p.to_path_buf())
}
