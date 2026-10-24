# fermaid

[English](README.md) | [繁體中文](README.zh-TW.md) | [简体中文](README.zh-CN.md) | **日本語**

fermaid は [Mermaid](https://mermaid.js.org) のダイアグラムを純粋な Rust で SVG に変換します。JavaScript ランタイム、ヘッドレスブラウザ、Node.js は不要です。小さく依存関係のない TypeScript 製レンダラー [pele](https://github.com/obsidianmd/pele) の移植で、Mermaid 12.1.0 の構文を対象とし、pele 独自のレイアウトとビジュアルスタイルを採用しています。

> **ステータス:** pre-alpha。基盤（前処理、テキストの計測と折り返し、SVG 出力、エラーと制限）は整っていますが、対応しているダイアグラムの種類はまだありません。現在はすべてのダイアグラムが `Unsupported` エラーを返します。最初に円グラフ、続いてフローチャートとシーケンス図に対応する予定です。[ロードマップ](docs/ROADMAP.md)を参照してください。

## 目標

- 純粋な Rust、同期処理、実行時の依存関係なし。ライブラリ自体は I/O を行いません。
- Mermaid 構文との互換性を保ちつつ、独自のすっきりしたレイアウトとビジュアルスタイルを採用。
- 決定的な出力：同じ入力とオプションからはバイト単位で同一の SVG を生成。
- テーマ変更が可能：色とフォントは SVG 内の CSS 変数。
- 信頼できない入力にも安全：テキストのエスケープ、リンクとスタイルのフィルタリング、サイズとネストの深さの制限。

## ライブラリ

```rust
let options = fermaid::RenderOptions {
    font_size: Some(14.0),
    ..Default::default()
};
match fermaid::render("pie\n  \"Rust\": 70\n  \"Other\": 30", &options) {
    Ok(rendered) => println!("{}", rendered.svg),
    Err(e) => eprintln!("{}:{}: {}\n{}", e.line, e.column, e.message, e.snippet),
}
```

`render` は SVG とそのサイズ・リンクを返すか、エラーコード（`Unsupported`、`Syntax`、`Semantic`、`Limit`）、行番号と列番号、問題箇所を示すスニペットを含む `Error` を返します。`detect_type` はレンダリングせずにダイアグラムの種類を判定します。ダイアグラムの種類ごとに Cargo feature があり、デフォルトの `all` feature にはすべての種類が含まれます。

## コマンドライン

```sh
cargo install --path crates/fermaid-cli
fermaid diagram.mmd -o diagram.svg
cat diagram.mmd | fermaid --font-size 14 --id-prefix fig1 > diagram.svg
```

## テーマ

SVG の色とフォントは CSS カスタムプロパティから取得され、組み込みのフォールバック値があります。そのため、ページは再レンダリングせずにダイアグラムのテーマを変更できます。

```css
svg {
  --fermaid-bg: #1e1e1e;
  --fermaid-fg: #dcddde;
  --fermaid-line: #999;
  --fermaid-accent: #7f6df2;
  --fermaid-font: "Inter", sans-serif;
}
```

使用できる変数：`--fermaid-bg`、`--fermaid-fg`、`--fermaid-muted`、`--fermaid-line`、`--fermaid-surface`、`--fermaid-surface-alt`、`--fermaid-border`、`--fermaid-accent`、`--fermaid-radius`、`--fermaid-font`、`--fermaid-font-mono`。グラフ系の種類では `--fermaid-series-1` から `--fermaid-series-8` が追加されます。

## 開発

```sh
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all
cargo +nightly fuzz run render    # cargo-fuzz が必要
```

サポートする最小の Rust バージョンは 1.85 です。

## クレジット

fermaid のアルゴリズム、テキスト計測データ、制限は、Obsidian チームの kepano による [pele](https://github.com/obsidianmd/pele)（MIT）から移植しています。ダイアグラムの構文と、テストに使うパーサー仕様は [Mermaid](https://github.com/mermaid-js/mermaid)（MIT）に由来します。詳しくは [NOTICE](NOTICE) を参照してください。

## ライセンス

[MIT](LICENSE)
