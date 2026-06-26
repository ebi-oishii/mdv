# Roadmap（フェーズ分割）

各フェーズの完了条件（Definition of Done）を明示。

## Phase 0 — 足場 ✓ 完了
- [x] Tauri 2 + SvelteKit (SPA) のプロジェクト雛形
- [x] `npm run check` と `cargo check` が通る
- [x] CI（GitHub Actions, Mac/Win/Linux）

## Phase 0.5 — ワークスペース化（1〜2日）
TUI 追加を見据えた Rust 側の再構成。

- [ ] ルートに `Cargo.toml` を作りワークスペース定義
- [ ] `crates/mdv-core/` を新設し、Doc/Diff/Git の型と関数を移管
- [ ] `src-tauri/` から `mdv-core` を path 依存で参照
- [ ] `crates/mdv-tui/` の空 binary を作成（Hello World 起動のみ）
- [ ] `cargo build --workspace` と既存 `npm run tauri dev` が共に通る
- [ ] CI を更新（matrix に `mdv-tui` ビルドも追加）

**DoD**: GUI と TUI 両バイナリが空っぽながらビルド・起動できる。

## Phase 1 — Source / Preview / 切替（1週間程度）
GUI と TUI 並行で進める。共通ロジックは `mdv-core` に集約。

### GUI
- [ ] ファイルを開く・保存する（tauri-plugin-fs + dialog）
- [ ] DocStore（Svelte runes）の実装
- [ ] SourceView（CodeMirror 6 + markdown）
- [ ] PreviewView（markdown-it + DOMPurify）
- [ ] ModeBar による切替
- [ ] スクロール位置同期（行番号ベース）

### TUI
- [ ] clap で引数パース（`mdv-tui [FILE] [--mode] [--read-only]`）
- [ ] AppState とイベントループ
- [ ] SourceView（edtui 評価 → 採否決定）
- [ ] PreviewView（termimad もしくは pulldown-cmark + ANSI）
- [ ] Tab でモード切替
- [ ] `:w` / `:q` の最小コマンドモード

**DoD**: GUI も TUI も 1MB の MD ファイルを開いて編集・保存でき、Preview を遅延なく往復できる。

## Phase 2 — Diff モード（1週間程度）
- [ ] `mdv-core::git::diff_against_head(path) -> Vec<HunkSummary>`
- [ ] GUI: DiffView（Highlight Only / Full サブモード）
- [ ] TUI: DiffView（同上、ANSI 色帯と緑赤）
- [ ] Diff 基準切替 UI（HEAD / index / 任意のリビジョン）

**DoD**: Git 管理下の MD を編集中に、HEAD との差分が両 UI の両サブモードで表示される。

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
- [ ] パッケージ署名と配布（GUI: dmg/msi/AppImage、TUI: cargo install / Homebrew formula）
