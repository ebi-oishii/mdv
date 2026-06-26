use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HunkKind {
    Added,
    Removed,
    Modified,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HunkSummary {
    pub start_line: usize,
    pub end_line: usize,
    pub kind: HunkKind,
}

pub fn line_diff(old: &str, new: &str) -> Vec<HunkSummary> {
    use similar::{ChangeTag, TextDiff};

    let diff = TextDiff::from_lines(old, new);
    let mut hunks: Vec<HunkSummary> = Vec::new();
    let mut current: Option<(usize, usize, HunkKind)> = None;

    for change in diff.iter_all_changes() {
        let Some(new_idx) = change.new_index() else {
            continue;
        };
        let line = new_idx + 1;

        let kind = match change.tag() {
            ChangeTag::Insert => Some(HunkKind::Added),
            ChangeTag::Delete => Some(HunkKind::Removed),
            ChangeTag::Equal => None,
        };

        match (current.as_mut(), kind) {
            (Some(cur), Some(k)) if cur.2 == k && cur.1 + 1 == line => cur.1 = line,
            (Some(_), Some(k)) => {
                let (s, e, prev) = current.take().unwrap();
                hunks.push(HunkSummary {
                    start_line: s,
                    end_line: e,
                    kind: prev,
                });
                current = Some((line, line, k));
            }
            (None, Some(k)) => current = Some((line, line, k)),
            (Some(_), None) => {
                let (s, e, prev) = current.take().unwrap();
                hunks.push(HunkSummary {
                    start_line: s,
                    end_line: e,
                    kind: prev,
                });
            }
            (None, None) => {}
        }
    }

    if let Some((s, e, k)) = current {
        hunks.push(HunkSummary {
            start_line: s,
            end_line: e,
            kind: k,
        });
    }

    hunks
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
    }

    #[test]
    fn unchanged_yields_no_hunks() {
        let s = "a\nb\nc\n";
        assert!(line_diff(s, s).is_empty());
    }
}
