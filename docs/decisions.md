# 技術選定 — 根拠とトレードオフ

各選定について「採用案 / 比較対象 / 決め手」を簡潔に記録する。

## ADR-001: アプリシェルに Tauri 2 を採用

- **採用**: Tauri 2.x（Rust + システム WebView）
- **比較**: Electron / Flutter / 各 OS ネイティブ
- **決め手**:
  - 「軽量」要件にもっとも合致（バイナリ・メモリとも Electron の数分の一）
  - Tauri 2 から iOS/Android 公式対応 → モバイル要件をクリア
  - Rust 側で `git2`, `similar` など堅牢な crate を活かせる
- **代償**:
  - WebView 差異（特に Linux WebKitGTK、iOS WKWebView）の検証コスト
  - Electron に比べ事例・解決済 SO 質問が少ない
- **却下理由**:
  - Electron: バイナリ 80MB+ がモバイル含めた "軽量" 方針と矛盾
  - Flutter: Markdown 周辺ライブラリの厚みが弱い、特に WYSIWYG 系が事実上ない
  - ネイティブ複数言語: 工数が許容外

## ADR-002: フロントは Svelte 5

- **採用**: Svelte 5（runes ベース、SvelteKit static または素の Svelte SPA）
- **比較**: React / Solid / Vue
- **決め手**:
  - ランタイムが極小、Tauri との組み合わせでロード時間を最小化
  - Runes による細粒度リアクティブは DocStore + 多 View の同期に素直
- **代償**:
  - Milkdown / TipTap は React 事例が多い。Svelte バインディングは framework-agnostic ラッパ経由になる
- **却下理由**:
  - React: ランタイムが重い、軽量要件と矛盾
  - Solid: API 自体は良いが MD エディタ統合事例が薄い

## ADR-003: ソースエディタに CodeMirror 6

- **採用**: CodeMirror 6（`@codemirror/lang-markdown` + 必要拡張のみ）
- **比較**: Monaco / textarea + 自前ハイライト
- **決め手**:
  - モジュラー設計でバンドルを必要分に絞れる
  - 拡張で diff デコレーション（Highlight Only モード）を素直に実装できる
  - モバイルでの動作実績あり
- **代償**:
  - 設定が宣言的で学習コストはある
- **却下理由**:
  - Monaco: 大きすぎる（数 MB）、デスクトップ前提のスタイル
  - 自前: スクロール、検索、折りたたみを再発明することになる

## ADR-004: WYSIWYG は Milkdown（Phase 3）

- **採用**: Milkdown（ProseMirror ベース、MD 双方向特化）
- **比較**: TipTap + markdown 拡張 / Lexical / 自前
- **決め手**:
  - 設計思想が "MD を一級市民として扱う" → Source ↔ WYSIWYG の往復に最適
  - プラグイン分割が細かく、必要機能だけロード可能
- **代償**:
  - ProseMirror 由来の API 複雑性
  - シリアライズで表記揺れが正規化される（`*` / `_` など）→ UI で明示する必要あり
- **却下理由**:
  - TipTap: WYSIWYG としては優秀だが MD への往復は別途実装する必要があり、結局 Milkdown と同等の手間
  - Lexical: 高性能だが MD 双方向のエコシステムが薄い

## ADR-005: Markdown パーサは markdown-it（表示）+ pulldown-cmark（必要時）

- **採用**:
  - フロントの Preview レンダリング: `markdown-it` + `markdown-it-gfm`
  - Rust 側で AST 解析が必要になった場合: `pulldown-cmark`
- **比較**: remark / micromark
- **決め手**:
  - markdown-it はプラグイン構造が枯れていて GFM・脚注など追加しやすい
  - Rust 側のパーサは差分のセクション単位スマート diff（将来）で役立つ
- **代償**: なし（フロントとバックで実装言語が違うため最低限の整合性検証は必要）

## ADR-006: Git 操作は git2 crate を Rust 側で

- **採用**: `git2`（libgit2 bindings）
- **比較**: `git` CLI を shell out / `gitoxide`
- **決め手**:
  - 外部 `git` 不在の環境でも動く
  - APIが安定、`diff_tree_to_workdir` 等で必要な情報が直接取れる
- **却下理由**:
  - CLI shell out: モバイルで使えない
  - `gitoxide`: まだ pure Rust 実装が完了しておらず、書き込み系が弱い

## ADR-007: 差分計算は similar crate

- **採用**: `similar`（Patience / Myers）
- **比較**: `diff` / 自前 LCS
- **決め手**:
  - GitHub と同等の Patience diff を選べる
  - ハンク単位の API が整っている
- **備考**: Highlight Only モードは「変更があった行範囲」のみ送れば十分なので、ハンクから `[startLine, endLine]` を抽出してフロントへ。

## ADR-008: TUI ライブラリは ratatui + crossterm

- **採用**:
  - メインフレーム: `ratatui` 0.29+
  - 端末バックエンド: `crossterm` 0.28+
  - 引数解析: `clap` v4 (derive)
- **比較**: cursive / Tauri TUI 拡張 / 自作
- **決め手**:
  - ratatui は Rust TUI のデファクト。広く採用され、ウィジェット・レイアウト DSL が成熟
  - immediate-mode 描画で AppState を 1 つ持てば良く、`mdv-core` の DocState と素直に結合できる
  - crossterm は Windows 含めて移植性◎、Tauri デスクトップが対応する全 OS で同じ挙動
- **代償**:
  - 編集ウィジェットは ratatui 純正には存在しない → `edtui` などサードパーティに依存、または自作
  - 日本語幅計算（東アジア全角・絵文字）は Rust エコシステム全体で罠が多く、テストでカバー必須
- **却下理由**:
  - cursive: 高レベルだが ratatui ほど活発でない。今回はカスタム描画（Highlight Only の色帯など）を多用するため低レベル制御が欲しい

## ADR-009: Rust コードは Cargo ワークスペース化し 3 crate に分割

- **採用**: ワークスペース構成
  - `src-tauri/` (member, package name = `mdv`) — Tauri GUI 用バイナリ
  - `crates/mdv-tui/` (package name = `mdv-tui`) — TUI バイナリ
  - `crates/mdv-core/` (package name = `mdv-core`) — UI 非依存の純粋ロジック
- **比較**: GUI と TUI を単一バイナリ + `--tui` フラグ / 別リポジトリ / GUI 内部に TUI モジュールを直書き
- **決め手**:
  - GUI と TUI は **UI レイヤーが完全に別** で共有意義が薄い一方、**MD パース・差分・Git** は完全に共通 → core 分離が自然
  - 別バイナリにすることで TUI を `cargo install mdv-tui` や Homebrew で単独配布できる（サーバ用途）
  - Tauri 既存の `src-tauri/` ディレクトリ名はツーリングの慣習なのでそのまま残し、ワークスペースの 1 メンバとして扱う
- **代償**:
  - ワークスペースルートに別途 `Cargo.toml` が必要
  - `mdv-core` の API 変更時に両 binary を更新する必要
- **却下理由**:
  - 単一バイナリ + フラグ: Tauri ランタイムを含むため TUI 用途でも数 MB のサイズ増。`tauri::Builder` を起動しない分岐パスの保守も面倒
  - 別リポジトリ: 共有コードを crate として公開する手間が割に合わない（個人プロジェクト規模）

## ADR-010: TUI の起動はデフォルトでフル機能、`--read-only` で読み取り専用

- **採用**: `mdv-tui file.md` は **編集モード** で起動。読み取り専用は `--read-only` で明示
- **比較**: 逆（デフォルト読み取り、`--edit` で編集）
- **決め手**:
  - GUI と TUI で挙動を揃える方が原則。GUI は常に編集可能
  - "less 代替" としての需要には `--read-only` で応えられる
- **代償**: 誤って書き換える事故の可能性は残る → 保存時は明示的に `Ctrl+S` か `:w` が必要なため実害は小さい

## ADR-011: スタイルは Tailwind ではなく素の CSS + CSS 変数

- **採用**: 素の CSS（コンポーネントスコープ） + CSS カスタムプロパティでテーマ
- **比較**: Tailwind / UnoCSS
- **決め手**:
  - Svelte の scoped style と相性が良くバンドル増を避けられる
  - テーマ切替（ライト/ダーク/エディタテーマ）は CSS 変数で完結する規模
- **却下理由**: Tailwind はクラス爆発と PurgeCSS の設定が、本規模では割に合わない
