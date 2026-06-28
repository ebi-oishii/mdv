<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { EditorState } from "@codemirror/state";
  import { EditorView, keymap, lineNumbers, highlightActiveLine } from "@codemirror/view";
  import { defaultKeymap, history, historyKeymap } from "@codemirror/commands";
  import { markdown } from "@codemirror/lang-markdown";
  import { syntaxHighlighting, defaultHighlightStyle } from "@codemirror/language";
  import { doc } from "$lib/stores/doc.svelte";
  import { mdvCmTheme } from "./cm-theme";

  let {
    text,
    onchange,
  }: { text: string; onchange: (t: string) => void } = $props();

  let container: HTMLDivElement;
  let view: EditorView | null = null;
  let lastEmitted = "";

  // Active-line extension overlay: paints `--mdv-active-line-bg` across the
  // full .source width at the current line's y. The right 3rem padding
  // strip lies outside cm-editor, so without this the highlight visibly
  // stops 3rem before the viewport edge. We position the .source::before
  // by writing CSS vars (--mdv-source-active-y / --mdv-source-active-h);
  // the actual paint happens in style {}.
  function updateActiveLine() {
    if (!view || !container) return;
    try {
      const head = view.state.selection.main.head;
      const block = view.lineBlockAt(head);
      const scrollerRect = view.scrollDOM.getBoundingClientRect();
      const sourceRect = container.getBoundingClientRect();
      // block.top is in CM's pre-scroll coordinates; subtract scrollTop to
      // get the rendered y inside the scroller, then translate into
      // .source's local coordinate space.
      const yInScroller = block.top - view.scrollDOM.scrollTop;
      const yInSource = scrollerRect.top + yInScroller - sourceRect.top;
      container.style.setProperty("--mdv-source-active-y", `${yInSource}px`);
      container.style.setProperty("--mdv-source-active-h", `${block.height}px`);
    } catch {
      // CM might not be ready or the editor is being torn down.
    }
  }

  onMount(() => {
    const state = EditorState.create({
      doc: text,
      extensions: [
        history(),
        lineNumbers(),
        highlightActiveLine(),
        syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
        keymap.of([...defaultKeymap, ...historyKeymap]),
        markdown(),
        EditorView.lineWrapping,
        mdvCmTheme,
        EditorView.updateListener.of((u) => {
          if (u.docChanged) {
            const next = u.state.doc.toString();
            lastEmitted = next;
            onchange(next);
          }
          if (u.selectionSet || u.docChanged || u.geometryChanged) {
            updateActiveLine();
          }
        }),
      ],
    });
    view = new EditorView({ state, parent: container });
    lastEmitted = text;
    // Paint the extension once on mount; future updates come from the
    // updateListener and the scrollDOM scroll handler below.
    requestAnimationFrame(updateActiveLine);
    view.scrollDOM.addEventListener("scroll", updateActiveLine, { passive: true });

    // Restore scroll position from DocStore so mode switches stay in place.
    // Defer one frame so CodeMirror has measured the layout.
    const restore = doc.currentLine;
    requestAnimationFrame(() => {
      if (!view) return;
      const total = view.state.doc.lines;
      const safe = Math.max(1, Math.min(total, restore));
      const pos = view.state.doc.line(safe).from;
      view.dispatch({ effects: EditorView.scrollIntoView(pos, { y: "start" }) });
    });
  });

  onDestroy(() => {
    view?.scrollDOM.removeEventListener("scroll", updateActiveLine);
    // Save topmost visible source line before tearing down so the next mode
    // can scroll there. posAtCoords is more reliable than lineBlockAtHeight
    // when the editor has padding/margins.
    if (view) {
      try {
        const rect = view.scrollDOM.getBoundingClientRect();
        const pos = view.posAtCoords({ x: rect.left + 8, y: rect.top + 4 });
        if (pos != null) {
          doc.currentLine = view.state.doc.lineAt(pos).number;
        }
      } catch {
        // best-effort; ignore if the layout isn't available
      }
    }
    view?.destroy();
  });

  $effect(() => {
    if (view && text !== lastEmitted) {
      lastEmitted = text;
      view.dispatch({
        changes: { from: 0, to: view.state.doc.length, insert: text },
      });
    }
  });
</script>

<div bind:this={container} class="source"></div>

<style>
  .source {
    height: 100%;
    overflow: hidden;
  }
  /* CodeMirror styling lives in $lib/views/cm-theme.ts (delivered via
     EditorView.theme so it beats CM defaults on specificity). Only
     editor-instance-specific bits remain here. */
  :global(.cm-editor) {
    font-family: ui-monospace, "SF Mono", Menlo, Consolas, monospace;
    font-size: var(--mdv-editor-font-size, 14px);
  }
  /* Reserve a right strip so long lines don't slide under the floating ☰
     button (top-right, 34px + 12px inset + shadow ≈ 54px).
     Padding goes on `.source` (the outer host), NOT on cm-scroller.
     CodeMirror computes line-wrap width from `cm-scroller.clientWidth`,
     and clientWidth *includes* padding — so padding on cm-scroller
     shrinks the visible box but leaves the wrap point unchanged, with
     cm-line happily rendering past cm-content's right edge. Putting the
     padding on `.source` makes cm-scroller itself narrower (its parent
     now reserves space), so clientWidth is the correct (shrunken) value
     and wrap actually fires earlier.
     Skipped in fullscreen: the 2.5rem top padding already moves content
     below the title overlay, and the ☰ menu sits next to the overlay
     in the OS-title-bar-free area, not on top of text. */
  :global(:root:not([data-fullscreen])) .source {
    padding-right: 3rem;
    box-sizing: border-box;
    /* Anchor the active-line extension ::before to .source. */
    position: relative;
  }
  /* Active-line extension: paints the highlight color across the full
     .source width (including the 3rem padding strip) at the current
     line's y. CM's own .cm-activeLine still paints inside cm-editor,
     and cm-editor's solid background sits *above* this ::before (later
     in document order, same stacking context), so the bar is visible
     only where cm-editor doesn't cover — i.e. the right 3rem strip.
     Position vars are written by updateActiveLine() in the script. */
  :global(:root:not([data-fullscreen])) .source::before {
    content: "";
    position: absolute;
    left: 0;
    right: 0;
    top: var(--mdv-source-active-y, -9999px);
    height: var(--mdv-source-active-h, 0);
    background: var(--mdv-active-line-bg);
    pointer-events: none;
  }
</style>
