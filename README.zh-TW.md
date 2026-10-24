# fermaid

[English](README.md) | **繁體中文** | [简体中文](README.zh-CN.md) | [日本語](README.ja.md)

fermaid 以純 Rust 將 [Mermaid](https://mermaid.js.org) 圖表轉換為 SVG，不需要 JavaScript 執行環境、無頭瀏覽器或 Node.js。它移植自 [pele](https://github.com/obsidianmd/pele)（一個小巧、無相依套件的 TypeScript 渲染器），支援 Mermaid 12.1.0 語法，並採用 pele 自己的版面配置與視覺風格。

> **狀態：** pre-alpha。已支援流程圖與圓餅圖；其他圖表類型目前會回傳 `Unsupported` 錯誤。接下來是循序圖。請參閱[開發路線圖](docs/ROADMAP.md)。

| 圖表類型 | 狀態 |
|---|---|
| 圓餅圖（`pie`） | 已支援：`showData`、`title`、`accTitle`、`accDescr`，以及 `pie` 設定項 `textPosition`、`donutHole` 與 `legendPosition` |
| 流程圖（`flowchart`、`graph`） | 已支援：所有節點形狀（含 `@{ shape: … }`）、各種連線與長度、連線 ID、可自訂方向的子圖、`classDef`、`class`、`style`、`linkStyle`、`click` 連結與提示文字、Markdown 標籤，以及 `flowchart` 設定項 `curve`、`nodeSpacing`、`rankSpacing` 與 `wrappingWidth` |
| 循序圖及其他 | 規劃中 |

## 目標

- 純 Rust、同步執行、沒有執行期相依套件，函式庫本身不做 I/O。
- 相容 Mermaid 語法，並採用自己簡潔的版面配置與視覺風格。
- 輸出具確定性：相同的輸入與選項會產生逐位元組相同的 SVG。
- 可自訂主題：顏色與字型是 SVG 中的 CSS 變數。
- 能安全處理不受信任的輸入：跳脫文字、過濾連結與樣式、限制大小與巢狀深度。

## 函式庫

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

`render` 會回傳 SVG 及其尺寸與連結，或回傳一個 `Error`，其中包含錯誤代碼（`Unsupported`、`Syntax`、`Semantic`、`Limit`）、行號與欄號，以及指出問題位置的程式碼片段。`detect_type` 不需渲染即可判斷圖表類型。將 `max_width` 設為宿主可用的寬度，能畫得更窄的圖表就會縮窄（圓餅圖會把圖例移到圖的下方）。`click` 陳述式中的連結會經過清理，且只保留允許的 scheme（`link_schemes`，預設為 `http`、`https`、`mailto` 與 `tel`）；橫向的流程圖在放不進 `max_width` 時會改為縱向。每種圖表類型都是一個 Cargo feature（`flowchart`、`pie`、…）；預設的 `all` feature 包含所有類型。

## 命令列

```sh
cargo install --path crates/fermaid-cli
fermaid diagram.mmd -o diagram.svg
cat diagram.mmd | fermaid --font-size 14 --max-width 360 --id-prefix fig1 > diagram.svg
```

## 主題

SVG 的顏色與字型取自 CSS 自訂屬性，並內建預設值，因此網頁不必重新渲染就能更換圖表主題：

```css
svg {
  --fermaid-bg: #1e1e1e;
  --fermaid-fg: #dcddde;
  --fermaid-line: #999;
  --fermaid-accent: #7f6df2;
  --fermaid-font: "Inter", sans-serif;
}
```

可用的變數：`--fermaid-bg`、`--fermaid-fg`、`--fermaid-muted`、`--fermaid-line`、`--fermaid-surface`、`--fermaid-surface-alt`、`--fermaid-border`、`--fermaid-accent`、`--fermaid-radius`、`--fermaid-font`、`--fermaid-font-mono`，以及用於圓餅圖扇形等資料序列顏色的 `--fermaid-series-1` 到 `--fermaid-series-8`。

## 開發

```sh
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all
cargo +nightly fuzz run render    # 需要 cargo-fuzz；另有 `pie` 與 `flowchart` 目標
cargo xtask compat pie --pele ../pele   # 從 pele 原始碼重新產生 Mermaid 規格測試案例
node tools/oracle/golden.mjs ../pele flowchart   # 重新產生 pele 對這些案例的解析結果
```

最低支援的 Rust 版本為 1.85。

## 致謝

fermaid 的演算法、文字量測資料與限制移植自 Obsidian 團隊 kepano 的 [pele](https://github.com/obsidianmd/pele)（MIT）。圖表語法以及作為測試的解析器規格來自 [Mermaid](https://github.com/mermaid-js/mermaid)（MIT）。詳見 [NOTICE](NOTICE)。

## 授權

[MIT](LICENSE)
