<script lang="ts">
  import type { BlameLine } from "$lib/types";

  /**
   * Per-line blame for `text`. `lines[i]` corresponds to line i+1 of
   * `text` (after splitting on newlines). Lines are rendered with a
   * compact gutter (sha · author · date) on the left and content on the
   * right, monospaced so the alignment reads like `git blame --line`.
   */
  let { text, lines }: { text: string; lines: BlameLine[] } = $props();

  // Strip the trailing empty string `split` produces for "foo\n" — the
  // blame data already drops it so the indices line up 1:1.
  const sourceLines = $derived.by(() => {
    const split = text.split("\n");
    if (split.length > 0 && split[split.length - 1] === "") split.pop();
    return split;
  });

  function pad2(n: number): string {
    return String(n).padStart(2, "0");
  }
  function formatDate(ts: number): string {
    if (!ts) return "";
    const d = new Date(ts * 1000);
    return `${d.getFullYear()}-${pad2(d.getMonth() + 1)}-${pad2(d.getDate())}`;
  }
  function tooltipFor(meta: BlameLine | undefined): string | undefined {
    if (!meta) return undefined;
    if (meta.origin === "git") {
      const parts: string[] = [];
      if (meta.sha) parts.push(meta.sha);
      if (meta.author) parts.push(meta.author);
      if (meta.email) parts.push(`<${meta.email}>`);
      if (meta.summary) parts.push(`\n${meta.summary}`);
      return parts.join(" ");
    }
    return meta.origin === "local" ? "Local snapshot" : "Uncommitted buffer edit";
  }
</script>

<div class="blame">
  {#each sourceLines as line, i (i)}
    {@const meta = lines[i]}
    <div
      class="row"
      class:git={meta?.origin === "git"}
      class:local={meta?.origin === "local"}
      class:buffer={meta?.origin === "buffer"}
    >
      <div class="gutter" title={tooltipFor(meta)}>
        <span class="sha">{meta?.short_sha ?? "—"}</span>
        <span class="author">{meta?.author ?? ""}</span>
        <span class="date">{meta ? formatDate(meta.date_ts) : ""}</span>
      </div>
      <span class="lineno">{i + 1}</span>
      <pre class="content">{line || " "}</pre>
    </div>
  {/each}
</div>

<style>
  .blame {
    flex: 1;
    min-height: 0;
    overflow: auto;
    font-family: ui-monospace, "SF Mono", Menlo, Consolas, monospace;
    font-size: 13px;
    line-height: 1.5;
    background: var(--mddiff-bg);
  }
  .row {
    display: flex;
    align-items: baseline;
    gap: 0.5rem;
    padding: 0 0.5rem;
    border-left: 3px solid transparent;
  }
  .row.git {
    border-left-color: var(--mddiff-accent);
  }
  .row.local {
    border-left-color: var(--mddiff-warning-fg, #b58900);
  }
  .row.buffer {
    border-left-color: var(--mddiff-text-subtle);
  }
  .row:hover {
    background: var(--mddiff-surface-hi);
  }
  /* Gutter is fixed-width so columns align across lines. ~14em fits
     "<sha8> <author<=10> <date>" comfortably; longer authors truncate. */
  .gutter {
    display: inline-flex;
    align-items: baseline;
    gap: 0.5rem;
    width: 16em;
    flex-shrink: 0;
    color: var(--mddiff-text-mute);
    user-select: none;
    overflow: hidden;
    white-space: nowrap;
  }
  .gutter .sha {
    color: var(--mddiff-text-subtle);
    width: 5em;
    flex-shrink: 0;
  }
  .gutter .author {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .row.local .gutter .author,
  .row.buffer .gutter .author {
    font-style: italic;
  }
  .gutter .date {
    color: var(--mddiff-text-subtle);
    width: 6em;
    flex-shrink: 0;
    text-align: right;
  }
  .lineno {
    width: 3em;
    text-align: right;
    color: var(--mddiff-text-subtle);
    flex-shrink: 0;
    user-select: none;
  }
  .content {
    margin: 0;
    flex: 1;
    min-width: 0;
    white-space: pre-wrap;
    word-break: break-word;
    color: var(--mddiff-text);
    font: inherit;
  }
</style>
