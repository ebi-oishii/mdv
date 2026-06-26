<script lang="ts">
  import { doc } from "$lib/stores/doc.svelte";
  import { gitFullDiff, gitHunks } from "$lib/ipc/git";
  import type { DiffLine, DiffSubmode, HunkSummary } from "$lib/types";
  import HighlightView from "./diff/HighlightView.svelte";
  import FullDiffView from "./diff/FullDiffView.svelte";

  let submode = $state<DiffSubmode>("highlight");
  let base = $state("HEAD");
  let baseInput = $state("HEAD");
  let hunks = $state<HunkSummary[]>([]);
  let lines = $state<DiffLine[]>([]);
  let error = $state<string | null>(null);
  let loading = $state(false);

  let timer: ReturnType<typeof setTimeout> | null = null;

  $effect(() => {
    void doc.path;
    void doc.text;
    void submode;
    void base;

    if (!doc.path || !doc.gitAvailable) return;
    if (timer) clearTimeout(timer);
    timer = setTimeout(load, 250);
    return () => {
      if (timer) clearTimeout(timer);
    };
  });

  async function load() {
    if (!doc.path) return;
    loading = true;
    error = null;
    try {
      if (submode === "highlight") {
        hunks = await gitHunks(doc.path, doc.text, base);
      } else {
        lines = await gitFullDiff(doc.path, doc.text, base);
      }
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  function applyBase(e: SubmitEvent) {
    e.preventDefault();
    base = baseInput.trim() || "HEAD";
  }

  const addedCount = $derived(
    hunks.reduce(
      (n, h) =>
        n + (h.kind === "added" || h.kind === "modified" ? h.end_line - h.start_line + 1 : 0),
      0,
    ),
  );
  const removedCount = $derived(hunks.reduce((n, h) => n + h.removed_count, 0));
</script>

<div class="diff-view">
  <div class="submode-bar">
    <div class="left">
      <div class="submode-toggle" role="tablist">
        <button
          role="tab"
          aria-selected={submode === "highlight"}
          class:active={submode === "highlight"}
          onclick={() => (submode = "highlight")}
        >
          Highlight Only
        </button>
        <button
          role="tab"
          aria-selected={submode === "full"}
          class:active={submode === "full"}
          onclick={() => (submode = "full")}
        >
          Full
        </button>
      </div>
      <form class="base-form" onsubmit={applyBase}>
        <span class="prefix">vs</span>
        <input
          type="text"
          bind:value={baseInput}
          spellcheck="false"
          autocomplete="off"
          placeholder="HEAD"
          aria-label="Compare base revision"
        />
        <button type="submit" disabled={baseInput === base}>Apply</button>
      </form>
    </div>
    <div class="meta">
      {#if loading}<span class="loading">…</span>{/if}
      {#if submode === "highlight" && !loading && !error}
        <span class="added">+{addedCount}</span>
        <span class="removed">−{removedCount}</span>
      {/if}
    </div>
  </div>

  {#if !doc.path}
    <div class="empty">No file open.</div>
  {:else if !doc.gitAvailable}
    <div class="empty">This file is not in a Git repository.</div>
  {:else if error}
    <div class="error">{error}</div>
  {:else if submode === "highlight"}
    <HighlightView text={doc.text} {hunks} />
  {:else}
    <FullDiffView {lines} />
  {/if}
</div>

<style>
  .diff-view {
    display: flex;
    flex-direction: column;
    height: 100%;
    min-height: 0;
  }
  .submode-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.5rem 1rem;
    border-bottom: 1px solid light-dark(#ddd, #333);
    background: light-dark(#fafafa, #222);
    flex-shrink: 0;
    font-size: 0.85rem;
  }
  .left {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex-wrap: wrap;
  }
  .submode-toggle {
    display: inline-flex;
    border: 1px solid light-dark(#ddd, #444);
    border-radius: 5px;
    overflow: hidden;
  }
  .submode-toggle button {
    background: transparent;
    border: 0;
    padding: 0.25rem 0.7rem;
    font: inherit;
    color: light-dark(#444, #ccc);
    cursor: pointer;
  }
  .submode-toggle button + button {
    border-left: 1px solid light-dark(#ddd, #444);
  }
  .submode-toggle button.active {
    background: light-dark(#e3eaf5, #2b3a55);
    color: light-dark(#16325c, #b9d0ff);
  }
  .base-form {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    color: light-dark(#666, #999);
  }
  .base-form .prefix {
    user-select: none;
  }
  .base-form input {
    width: 9em;
    padding: 0.2rem 0.5rem;
    font: inherit;
    font-family: ui-monospace, "SF Mono", Menlo, monospace;
    background: light-dark(#fff, #1a1a1a);
    color: inherit;
    border: 1px solid light-dark(#ccc, #444);
    border-radius: 4px;
  }
  .base-form button {
    background: transparent;
    border: 1px solid light-dark(#ccc, #444);
    border-radius: 4px;
    padding: 0.2rem 0.6rem;
    font: inherit;
    color: inherit;
    cursor: pointer;
  }
  .base-form button:hover:not(:disabled) {
    background: light-dark(#eee, #2a2a2a);
  }
  .base-form button:disabled {
    opacity: 0.4;
    cursor: default;
  }
  .meta {
    display: flex;
    gap: 0.6rem;
    font-family: ui-monospace, monospace;
  }
  .added {
    color: #2ea043;
  }
  .removed {
    color: #cf222e;
  }
  .loading {
    color: light-dark(#888, #666);
  }
  .empty,
  .error {
    padding: 2rem;
    text-align: center;
    color: light-dark(#888, #aaa);
  }
  .error {
    color: light-dark(#a33, #ffb4b4);
  }
</style>
