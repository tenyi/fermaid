# fermaid

[English](README.md) | [繁體中文](README.zh-TW.md) | **简体中文** | [日本語](README.ja.md)

fermaid 使用纯 Rust 将 [Mermaid](https://mermaid.js.org) 图表转换为 SVG，无需 JavaScript 运行时、无头浏览器或 Node.js。

> **状态：** 早期开发中，尚未提供可用的 API。

## 目标

- 纯 Rust、同步执行、依赖最少。
- 兼容 Mermaid 语法，并采用自己简洁的布局与视觉风格。
- 可自定义主题：颜色与字体以 CSS 变量的形式写入 SVG。
- 可安全处理不受信任的输入：转义文本、过滤链接与样式、限制大小。

## 灵感来源

fermaid 受到 Obsidian 团队的 [Pele](https://github.com/obsidianmd/pele) 启发。Pele 是一个小巧、无依赖的 TypeScript 库，能将 Mermaid 图表转为 SVG。fermaid 的目标是把同样的做法带入 Rust 生态。

## 开发

```sh
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all
```

## 许可证

[MIT](LICENSE)
