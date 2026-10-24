# fermaid

[English](README.md) | **繁體中文** | [简体中文](README.zh-CN.md) | [日本語](README.ja.md)

fermaid 以純 Rust 將 [Mermaid](https://mermaid.js.org) 圖表轉換為 SVG，不需要 JavaScript 執行環境、無頭瀏覽器或 Node.js。

> **狀態：** 早期開發中，尚未提供可用的 API。

## 目標

- 純 Rust、同步執行、相依套件最少。
- 相容 Mermaid 語法，並採用自己簡潔的版面配置與視覺風格。
- 可自訂主題：顏色與字型以 CSS 變數的形式寫入 SVG。
- 可安全處理不受信任的輸入：跳脫文字、過濾連結與樣式、限制大小。

## 靈感來源

fermaid 受到 Obsidian 團隊的 [Pele](https://github.com/obsidianmd/pele) 啟發。Pele 是一個小巧、無相依套件的 TypeScript 函式庫，能將 Mermaid 圖表轉為 SVG。fermaid 的目標是把同樣的做法帶進 Rust 生態系。

## 開發

```sh
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all
```

## 授權

[MIT](LICENSE)
