# fermaid

[English](README.md) | [繁體中文](README.zh-TW.md) | [简体中文](README.zh-CN.md) | **日本語**

fermaid は [Mermaid](https://mermaid.js.org) のダイアグラムを純粋な Rust で SVG に変換するライブラリです。JavaScript ランタイム、ヘッドレスブラウザ、Node.js は不要です。

> **ステータス:** 開発初期段階です。まだ利用可能な API はありません。

## 目標

- 純粋な Rust、同期処理、最小限の依存関係。
- Mermaid の構文に対応しつつ、独自のシンプルなレイアウトとビジュアルスタイルを採用。
- テーマ変更が可能：色とフォントを SVG 内の CSS 変数として公開。
- 信頼できない入力にも安全：テキストのエスケープ、リンクとスタイルのフィルタリング、サイズ制限。

## インスピレーション

fermaid は Obsidian チームの [Pele](https://github.com/obsidianmd/pele) に触発されています。Pele は Mermaid のダイアグラムを SVG に変換する、依存関係のない小さな TypeScript ライブラリです。fermaid は同じアプローチを Rust のエコシステムにもたらすことを目指しています。

## 開発

```sh
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all
```

## ライセンス

[MIT](LICENSE)
