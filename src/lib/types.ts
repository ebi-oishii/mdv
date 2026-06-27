export type Mode = "source" | "preview" | "diff";

export type HunkKind = "added" | "modified" | "removed";

export interface HunkSummary {
  kind: HunkKind;
  start_line: number;
  end_line: number;
  removed_count: number;
}

export type DiffLine =
  | { kind: "equal"; old_no: number; new_no: number; text: string }
  | { kind: "added"; new_no: number; text: string }
  | { kind: "removed"; old_no: number; text: string };

export type DiffSubmode = "highlight" | "full";

export type BaseKind = "special" | "branch" | "tag" | "commit";

/**
 * - `differs`: file at this revision differs from current buffer AND is the
 *   canonical entry of its same-content run
 * - `identical`: file at this revision is byte-identical to current buffer
 * - `redundant`: a more recent commit shown above has the same content;
 *   choosing this gives the same diff, so the marker is suppressed
 * - `unknown`: couldn't determine (rare)
 */
export type DiffMarker = "differs" | "identical" | "redundant" | "unknown";

export interface BaseOption {
  revspec: string;
  label: string;
  kind: BaseKind;
  detail: string | null;
  marker: DiffMarker;
}
