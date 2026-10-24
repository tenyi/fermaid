# fermaid

[English](README.md) | [繁體中文](README.zh-TW.md) | **简体中文** | [日本語](README.ja.md)

fermaid 以纯 Rust 将 [Mermaid](https://mermaid.js.org) 图表转换为 SVG，无需 JavaScript 运行时、无头浏览器或 Node.js。它移植自 [pele](https://github.com/obsidianmd/pele)（一个小巧、无依赖的 TypeScript 渲染器），支持 Mermaid 12.1.0 语法，并采用 pele 自己的布局与视觉风格。

> **状态：** pre-alpha。基础架构已完成（预处理、文本测量与换行、SVG 输出、错误与限制），但尚未支持任何图表类型：目前所有图表都会返回 `Unsupported` 错误。饼图会最先支持，随后是流程图和时序图。请参阅[开发路线图](docs/ROADMAP.md)。

## 目标

- 纯 Rust、同步执行、没有运行时依赖，库本身不做 I/O。
- 兼容 Mermaid 语法，并采用自己简洁的布局与视觉风格。
- 输出具有确定性：相同的输入和选项会生成逐字节相同的 SVG。
- 可自定义主题：颜色和字体是 SVG 中的 CSS 变量。
- 能安全处理不受信任的输入：转义文本、过滤链接和样式、限制大小与嵌套深度。

## 库

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

`render` 返回 SVG 及其尺寸和链接，或返回一个 `Error`，其中包含错误代码（`Unsupported`、`Syntax`、`Semantic`、`Limit`）、行号和列号，以及指出问题位置的代码片段。`detect_type` 无需渲染即可判断图表类型。每种图表类型都是一个 Cargo feature；默认的 `all` feature 包含所有类型。

## 命令行

```sh
cargo install --path crates/fermaid-cli
fermaid diagram.mmd -o diagram.svg
cat diagram.mmd | fermaid --font-size 14 --id-prefix fig1 > diagram.svg
```

## 主题

SVG 的颜色和字体取自 CSS 自定义属性，并内置默认值，因此网页无需重新渲染即可更换图表主题：

```css
svg {
  --fermaid-bg: #1e1e1e;
  --fermaid-fg: #dcddde;
  --fermaid-line: #999;
  --fermaid-accent: #7f6df2;
  --fermaid-font: "Inter", sans-serif;
}
```

可用的变量：`--fermaid-bg`、`--fermaid-fg`、`--fermaid-muted`、`--fermaid-line`、`--fermaid-surface`、`--fermaid-surface-alt`、`--fermaid-border`、`--fermaid-accent`、`--fermaid-radius`、`--fermaid-font`、`--fermaid-font-mono`。图表类型会再加入 `--fermaid-series-1` 到 `--fermaid-series-8`。

## 开发

```sh
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all
cargo +nightly fuzz run render    # 需要 cargo-fuzz
```

最低支持的 Rust 版本为 1.85。

## 致谢

fermaid 的算法、文本测量数据与限制移植自 Obsidian 团队 kepano 的 [pele](https://github.com/obsidianmd/pele)（MIT）。图表语法以及用作测试的解析器规范来自 [Mermaid](https://github.com/mermaid-js/mermaid)（MIT）。详见 [NOTICE](NOTICE)。

## 许可证

[MIT](LICENSE)
