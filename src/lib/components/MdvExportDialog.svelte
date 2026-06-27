<script lang="ts">
  import { gitListBases } from "$lib/ipc/git";
  import { mdvPack } from "$lib/ipc/mdv";
  import { pickSavePath, writeFile } from "$lib/ipc/fs";
  import type { BaseKind, BaseOption } from "$lib/types";

  let {
    path,
    currentText,
    onSaved,
    onCancel,
  }: {
    path: string;
    currentText: string;
    onSaved: (msg: string) => void;
    onCancel: () => void;
  } = $props();

  let bases = $state<BaseOption[]>([]);
  let selected = $state<string>("HEAD");
  let loading = $state(false);
  let error = $state<string | null>(null);

  $effect(() => {
    gitListBases(path, currentText)
      .then((b) => {
        bases = b;
      })
      .catch((e) => {
        error = String(e);
      });
  });

  function byKind(kind: BaseKind): BaseOption[] {
    return bases.filter((b) => b.kind === kind);
  }

  function label(b: BaseOption): string {
    return b.label;
  }

  async function save() {
    loading = true;
    error = null;
    try {
      const out = await pickSavePath("mdv", "mdv package", path);
      if (!out) {
        loading = false;
        return;
      }
      const packed = await mdvPack(path, currentText, selected);
      await writeFile(out, packed.content);
      onSaved(
        `.mdv saved: ${packed.commit_count} commits, ${packed.snapshot_count} snapshots, ${formatBytes(packed.bundle_bytes)} compressed`,
      );
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  function formatBytes(n: number): string {
    if (n < 1024) return `${n} B`;
    if (n < 1024 * 1024) return `${(n / 1024).toFixed(1)} KB`;
    return `${(n / (1024 * 1024)).toFixed(2)} MB`;
  }
</script>

<div class="overlay" role="presentation" onclick={onCancel}>
  <div
    class="modal"
    role="dialog"
    tabindex="-1"
    aria-modal="true"
    aria-labelledby="mdv-export-title"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => e.key === "Escape" && onCancel()}
  >
    <h2 id="mdv-export-title">Export .mdv with history</h2>
    <p class="help">
      Bundle the file's history from the chosen base up to the current buffer into a
      self-contained <code>.mdv</code>. The receiver can read it as plain Markdown,
      and your local Git stays untouched.
    </p>

    <label class="row">
      <span>Include history since:</span>
      <select bind:value={selected} disabled={loading}>
        <optgroup label="Special">
          {#each byKind("special") as b}
            <option value={b.revspec}>{label(b)}</option>
          {/each}
        </optgroup>
        {#if byKind("branch").length > 0}
          <optgroup label="Branches">
            {#each byKind("branch") as b}
              <option value={b.revspec}>{label(b)}</option>
            {/each}
          </optgroup>
        {/if}
        {#if byKind("tag").length > 0}
          <optgroup label="Tags">
            {#each byKind("tag") as b}
              <option value={b.revspec}>{label(b)}</option>
            {/each}
          </optgroup>
        {/if}
        {#if byKind("commit").length > 0}
          <optgroup label="Recent commits">
            {#each byKind("commit") as b}
              <option value={b.revspec}>{label(b)}</option>
            {/each}
          </optgroup>
        {/if}
      </select>
    </label>

    {#if error}
      <div class="error">{error}</div>
    {/if}

    <div class="actions">
      <button type="button" onclick={onCancel} disabled={loading}>Cancel</button>
      <button type="button" class="primary" onclick={save} disabled={loading}>
        {loading ? "Packing…" : "Save .mdv"}
      </button>
    </div>
  </div>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
    padding: 1rem;
  }
  .modal {
    background: light-dark(#fff, #1e1e1e);
    color: inherit;
    border-radius: 8px;
    padding: 1.25rem 1.5rem;
    max-width: 36em;
    width: 100%;
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.35);
  }
  h2 {
    margin: 0 0 0.5rem;
    font-size: 1.1rem;
  }
  .help {
    margin: 0 0 1rem;
    font-size: 0.88rem;
    color: light-dark(#555, #aaa);
    line-height: 1.5;
  }
  .help code {
    font-family: ui-monospace, "SF Mono", Menlo, monospace;
    font-size: 0.92em;
    padding: 0 0.25em;
    background: light-dark(#f3f3f3, #2a2a2a);
    border-radius: 3px;
  }
  .row {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin: 0.75rem 0;
    font-size: 0.9rem;
  }
  .row span {
    flex-shrink: 0;
  }
  .row select {
    flex: 1;
    font: inherit;
    padding: 0.25rem 0.5rem;
    background: light-dark(#fff, #1a1a1a);
    color: inherit;
    border: 1px solid light-dark(#ccc, #444);
    border-radius: 4px;
  }
  .error {
    padding: 0.5rem 0.7rem;
    background: light-dark(#fff0f0, #4a2222);
    color: light-dark(#a33, #ffb4b4);
    border-radius: 4px;
    font-size: 0.85rem;
    margin: 0.5rem 0;
  }
  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.5rem;
    margin-top: 1rem;
  }
  .actions button {
    background: transparent;
    border: 1px solid light-dark(#ccc, #555);
    border-radius: 5px;
    padding: 0.4rem 1rem;
    font: inherit;
    color: inherit;
    cursor: pointer;
  }
  .actions button:hover:not(:disabled) {
    background: light-dark(#eee, #2a2a2a);
  }
  .actions button.primary {
    background: light-dark(#16325c, #2b3a55);
    color: #fff;
    border-color: transparent;
  }
  .actions button.primary:hover:not(:disabled) {
    background: light-dark(#0d2440, #3a4a6b);
  }
  .actions button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
