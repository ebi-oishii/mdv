import { invoke } from "@tauri-apps/api/core";

export interface PackResponse {
  content: string;
  commit_count: number;
  snapshot_count: number;
  bundle_bytes: number;
}

export async function mdvPack(
  path: string,
  currentText: string,
  base: string,
): Promise<PackResponse> {
  return await invoke<PackResponse>("mdv_pack", {
    path,
    currentText,
    base,
  });
}

/**
 * Strip the `<!-- mdv:v1 ... -->` package block from a `.mdv` file's content
 * and return the markdown body. Returns the input unchanged when no block is
 * present, so it's safe to call on any text.
 */
export async function mdvExtractBody(content: string): Promise<string> {
  return await invoke<string>("mdv_extract_body", { content });
}
