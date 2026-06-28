import { EditorView } from "@codemirror/view";
import type { EditorState } from "@codemirror/state";
import { syntaxTree } from "@codemirror/language";
import type { SyntaxNode } from "@lezer/common";
import {
  routeHref,
  modifierPressed,
  type LinkClickContext,
} from "./link-click";

/**
 * CodeMirror DOM-event extension that lets users open links by ⌘/Ctrl-clicking
 * on link syntax inside Source / Live Preview. Without this, there are no
 * actual `<a>` elements in the editor (it's all decorated text), so the
 * regular DOM click handler used by Preview / Diff / WYSIWYG can't fire.
 *
 * Plain clicks still position the caret as normal. The modifier requirement
 * matches the convention used by other editable views.
 */
export function linkClickCmExtension(ctx: LinkClickContext) {
  return EditorView.domEventHandlers({
    mousedown(event, view) {
      if (!modifierPressed(event)) return false;

      const pos = view.posAtCoords({ x: event.clientX, y: event.clientY });
      if (pos == null) return false;

      const href = urlAtPos(view.state, pos);
      if (!href) return false;

      // Prevent CodeMirror's own mousedown from running, which would
      // otherwise move the caret to the click position before our async
      // navigation kicks in.
      event.preventDefault();
      routeHref(href, ctx);
      return true;
    },
  });
}

/**
 * Resolve `pos` to a link URL if it falls inside a Markdown `Link` or
 * `Image` node. Walks up the syntax tree to the enclosing link node, then
 * down to its `URL` child to extract the actual URL text.
 *
 * Returns `null` when the position isn't inside a link node, or when the
 * link is malformed enough that no URL child exists (rare in well-formed
 * Markdown).
 */
function urlAtPos(state: EditorState, pos: number): string | null {
  const tree = syntaxTree(state);
  let node: SyntaxNode | null = tree.resolveInner(pos, 0);
  while (node && node.name !== "Link" && node.name !== "Image") {
    node = node.parent;
  }
  if (!node) return null;

  let child = node.firstChild;
  while (child) {
    if (child.name === "URL") {
      return state.sliceDoc(child.from, child.to);
    }
    child = child.nextSibling;
  }
  return null;
}
