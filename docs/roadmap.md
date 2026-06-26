# Roadmap（フェーズ分割）

各フェーズの完了条件（Definition of Done）を明示。

## Phase 0 — 足場 ✓ 完了
- [x] Tauri 2 + SvelteKit (SPA) のプロジェクト雛形
- [x] `npm run check` と `cargo check` が通る
- [x] CI（GitHub Actions, Mac/Win/Linux）

## Phase 0.5 — ワークスペース化 ✓ 完了
- [x] ルート `Cargo.toml` でワークスペース定義
- [x] `crates/mdv-core/` を新設し型と共通ロジックを配置
- [x] `src-tauri/` から `mdv-core` を path 依存
- [x] `crates/mdv-tui/` のスケルトン（clap + ratatui Hello）
- [x] `cargo build --workspace` 成功、CI を workspace 対応

## Phase 1 — Source / Preview / 切替 ✓ 完了
### GUI ✓
- [x] ファイル開閉（custom Rust command + plugin-dialog）
- [x] DocStore（Svelte 5 runes）
- [x] SourceView（CodeMirror 6）
- [x] PreviewView（markdown-it + DOMPurify）
- [x] ModeBar 切替、⌘O/S/1/2 のショートカット
- [ ] スクロール位置同期（行番号ベース） — Phase 5 へ先送り

### TUI ✓
- [x] clap + crossterm + ratatui の足場
- [x] SourceView（tui-textarea 0.7）
- [x] PreviewView（pulldown-cmark → ratatui Text 自作レンダラ）
- [x] Ctrl+E でモード巡回、Ctrl+S 保存、Ctrl+Q 終了
- [ ] `:w` `:q` コマンドモード — Phase 5 へ先送り

## Phase 2 — Diff モード（Highlight Only / Full） ✓ 完了
- [x] `mdv-core` に `line_diff` / `full_diff` / git ラッパ
- [x] GUI DiffView（Highlight Only と Full、debounce 自動更新）
- [x] TUI DiffView（同上、ratatui で色付け）
- [x] ModeBar に Diff タブ、Git 配下でないと disabled

## Phase 2.5 — Side-by-Side Diff（追加）
GUI / TUI に第 3 サブモードを追加。設計は [docs/design.md §3](design.md) と
[ADR-011〜013](decisions.md) を参照。

### 共通基盤
- [ ] `HunkSummary` を `{ kind, new_start, new_end, old_start, old_end }` に拡張
- [ ] 既存 Highlight Only / Full の派生計算を新フィールド経由に置換
- [ ] テスト追加（Added/Modified/Removed の両側行範囲を検証）

### GUI Side-by-Side
- [ ] `mdv-core::git` に「OLD ブロブを取得して返す」関数を追加
- [ ] Tauri command `git_side_by_side(path, current_text)` で
  `{ old_text, new_text, hunks }` を返す
- [ ] markdown-it の token.map をフックして `class="mdv-changed-{kind}"` を注入
- [ ] `views/diff/SideBySideView.svelte`：2 ペイン、独立スクロール、
  PreviewView と同じスタイル + 変更ブロック背景色
- [ ] DiffView サブトグルに「Side-by-Side」追加

### TUI Side-by-Side（簡易版）
- [ ] `views/diff.rs` に `Submode::SideBySide` を追加
- [ ] 横幅 < 100 桁では巡回時にスキップ
- [ ] 左ペイン = OLD Source、右ペイン = NEW Source、各々で色付け
- [ ] スクロールは独立

**DoD**: GUI でレビュー的に「変わった箇所がレンダリングで見比べられる」、TUI でも
80〜200 桁の端末で並列表示が機能する。

## Phase 3 — WYSIWYG（GUI のみ、2週間程度）
- [ ] Milkdown 統合（Svelte ラッパ）
- [ ] DocStore との双方向バインド
- [ ] CJK / リスト / コードブロック / 画像の round-trip 検証
- [ ] 表記正規化が起きるケースのユーザ通知

**DoD**: WYSIWYG モードで編集した結果が Source モードに反映され、ファイルとして保存できる。

## Phase 4 — モバイル（並行可、別ブランチ）
- [ ] iOS / Android で GUI 起動
- [ ] レスポンシブ UI（ボトムタブ）
- [ ] OS のファイル選択経由でドキュメントを開く
- [ ] Git は読み取り（diff）のみ
- ※ TUI バイナリはモバイル配布対象外

**DoD**: TestFlight / 内部配布で実機動作確認。

## Phase 5 — 仕上げ
- [ ] テーマ（ライト/ダーク/エディタ配色、TUI は ANSI カラースキーム）
- [ ] 設定画面
- [ ] 大容量ファイルガード
- [ ] エラー時の挙動（ファイル消失、Git なしリポジトリなど）
- [ ] 先送り項目: GUI スクロール同期、TUI コマンドモード、TUI Diff キャッシュ、
  Side-by-Side の文字単位 inline diff、ペイン間スクロール同期
- [ ] パッケージ署名と配布（GUI: dmg/msi/AppImage、TUI: cargo install / Homebrew formula）
