import { invoke } from "@tauri-apps/api/core";
import type { DiffLine, HunkSummary } from "$lib/types";

export async function gitIsRepo(path: string): Promise<boolean> {
  return await invoke<boolean>("git_is_repo", { path });
}

export async function gitHunks(
  path: string,
  currentText: string,
): Promise<HunkSummary[]> {
  return await invoke<HunkSummary[]>("git_hunks", { path, currentText });
}

export async function gitFullDiff(
  path: string,
  currentText: string,
): Promise<DiffLine[]> {
  return await invoke<DiffLine[]>("git_full_diff", { path, currentText });
}
