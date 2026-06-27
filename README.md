# mdv

軽量・クロスプラットフォームの Markdown ビューア兼エディタ。

- Desktop (Mac / Windows / Linux) + Mobile (iOS / Android)
- Source / Live Preview / WYSIWYG / Preview の編集・閲覧モード
- Git 管理下のファイルでは GitHub 風の差分ビュー、または変更箇所のハイライトのみの軽量ビュー

## 開発

### GUI（Tauri）
```sh
npm install
npm run tauri dev           # Desktop
npm run tauri ios dev       # iOS (要 Xcode + iOS targets)
npm run tauri android dev   # Android (要 Android Studio + NDK + JAVA_HOME)
```

初回 iOS / Android 用には `npm run tauri ios init` / `npm run tauri android init` が必要。

### TUI
```sh
cargo run -p mdv-tui                       # ファイル指定なし
cargo run -p mdv-tui -- README.md          # ファイル指定
cargo run -p mdv-tui -- --help             # オプション一覧
```

### 品質チェック

```sh
npm run check              # Svelte + TypeScript
cargo check --workspace    # Rust 全 crate
cargo test  --workspace    # Rust テスト
```

## ドキュメント

- [docs/design.md](docs/design.md) — アーキテクチャと設計
- [docs/decisions.md](docs/decisions.md) — 技術選定の根拠とトレードオフ
- [docs/roadmap.md](docs/roadmap.md) — フェーズ分割
- [docs/tooling-research.md](docs/tooling-research.md) — 既存ツールから取り入れる使用感の整理
