class DocStore {
  text = $state("");
  path = $state<string | null>(null);
  savedText = $state("");
  gitAvailable = $state(false);

  get dirty() {
    return this.text !== this.savedText;
  }

  setText(t: string) {
    this.text = t;
  }

  load(path: string | null, text: string, gitAvailable: boolean) {
    this.path = path;
    this.text = text;
    this.savedText = text;
    this.gitAvailable = gitAvailable;
  }

  markSaved() {
    this.savedText = this.text;
  }

  /**
   * Called when WYSIWYG (Milkdown) round-trips the document on mount and
   * yields a normalized form (e.g. `*foo*` -> `_foo_`, link reference
   * expansion). If the user had no unsaved edits at that point, we treat the
   * normalized form as the new baseline so the dirty indicator doesn't
   * appear out of nowhere just from opening WYSIWYG. If they already had
   * unsaved edits, preserve dirty — the normalization is part of those edits.
   *
   * Side effect: `savedText` may diverge from the bytes actually on disk in
   * the no-prior-edits case. We accept that trade-off so the UX makes sense.
   */
  adoptNormalized(normalized: string) {
    if (this.text === this.savedText) {
      this.text = normalized;
      this.savedText = normalized;
    } else {
      this.text = normalized;
    }
  }
}

export const doc = new DocStore();
