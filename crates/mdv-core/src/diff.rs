use serde::{Deserialize, Serialize};
use similar::{ChangeTag, DiffOp, TextDiff};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HunkKind {
    Added,
    Modified,
    Removed,
}

/// Compact descriptor for the Highlight Only view.
/// Line numbers are 1-based and refer to the NEW buffer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HunkSummary {
    pub kind: HunkKind,
    /// For Added/Modified: first changed new-line. For Removed:
    /// the new-line immediately before the deletion (0 = top of file).
    pub start_line: usize,
    /// For Added/Modified: last changed new-line (inclusive).
    /// For Removed: same as `start_line`.
    pub end_line: usize,
    /// Number of old lines that were removed or replaced (0 for pure Added).
    pub removed_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum DiffLine {
    Equal {
        old_no: usize,
        new_no: usize,
        text: String,
    },
    Added {
        new_no: usize,
        text: String,
    },
    Removed {
        old_no: usize,
        text: String,
    },
}

pub fn line_diff(old: &str, new: &str) -> Vec<HunkSummary> {
    let diff = TextDiff::from_lines(old, new);
    let mut hunks = Vec::new();

    for op in diff.ops().iter().copied() {
        match op {
            DiffOp::Equal { .. } => {}
            DiffOp::Insert {
                new_index, new_len, ..
            } => hunks.push(HunkSummary {
                kind: HunkKind::Added,
                start_line: new_index + 1,
                end_line: new_index + new_len,
                removed_count: 0,
            }),
            DiffOp::Delete {
                old_len, new_index, ..
            } => hunks.push(HunkSummary {
                kind: HunkKind::Removed,
                start_line: new_index,
                end_line: new_index,
                removed_count: old_len,
            }),
            DiffOp::Replace {
                old_len,
                new_index,
                new_len,
                ..
            } => hunks.push(HunkSummary {
                kind: HunkKind::Modified,
                start_line: new_index + 1,
                end_line: new_index + new_len,
                removed_count: old_len,
            }),
        }
    }

    hunks
}

pub fn full_diff(old: &str, new: &str) -> Vec<DiffLine> {
    let diff = TextDiff::from_lines(old, new);
    let mut out = Vec::new();

    for change in diff.iter_all_changes() {
        let text = change
            .value()
            .strip_suffix('\n')
            .unwrap_or(change.value())
            .to_string();
        match change.tag() {
            ChangeTag::Equal => out.push(DiffLine::Equal {
                old_no: change.old_index().unwrap() + 1,
                new_no: change.new_index().unwrap() + 1,
                text,
            }),
            ChangeTag::Insert => out.push(DiffLine::Added {
                new_no: change.new_index().unwrap() + 1,
                text,
            }),
            ChangeTag::Delete => out.push(DiffLine::Removed {
                old_no: change.old_index().unwrap() + 1,
                text,
            }),
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_added_line() {
        let old = "a\nb\n";
        let new = "a\nb\nc\n";
        let hunks = line_diff(old, new);
        assert_eq!(hunks.len(), 1);
        assert_eq!(hunks[0].kind, HunkKind::Added);
        assert_eq!(hunks[0].start_line, 3);
        assert_eq!(hunks[0].end_line, 3);
    }

    #[test]
    fn detects_removed_line() {
        let old = "a\nb\nc\n";
        let new = "a\nc\n";
        let hunks = line_diff(old, new);
        assert_eq!(hunks.len(), 1);
        assert_eq!(hunks[0].kind, HunkKind::Removed);
        assert_eq!(hunks[0].start_line, 1);
        assert_eq!(hunks[0].removed_count, 1);
    }

    #[test]
    fn detects_modified_line() {
        let old = "a\nb\nc\n";
        let new = "a\nB\nc\n";
        let hunks = line_diff(old, new);
        assert_eq!(hunks.len(), 1);
        assert_eq!(hunks[0].kind, HunkKind::Modified);
        assert_eq!(hunks[0].start_line, 2);
        assert_eq!(hunks[0].end_line, 2);
    }

    #[test]
    fn unchanged_yields_no_hunks() {
        let s = "a\nb\nc\n";
        assert!(line_diff(s, s).is_empty());
    }

    #[test]
    fn full_diff_marks_each_line() {
        let old = "a\nb\n";
        let new = "a\nB\n";
        let lines = full_diff(old, new);
        assert!(matches!(lines[0], DiffLine::Equal { .. }));
        assert!(matches!(lines[1], DiffLine::Removed { .. }));
        assert!(matches!(lines[2], DiffLine::Added { .. }));
    }
}
