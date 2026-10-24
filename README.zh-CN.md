# fermaid

[English](README.md) | [繁體中文](README.zh-TW.md) | **简体中文** | [日本語](README.ja.md)

fermaid 以纯 Rust 将 [Mermaid](https://mermaid.js.org) 图表转换为 SVG，无需 JavaScript 运行时、无头浏览器或 Node.js。它移植自 [pele](https://github.com/obsidianmd/pele)（一个小巧、无依赖的 TypeScript 渲染器），支持 Mermaid 12.1.0 语法，并采用 pele 自己的布局与视觉风格。

> **状态：** pre-alpha。已支持流程图、时序图、状态图、类图、ER 图、需求图和饼图；其他图表类型目前会返回 `Unsupported` 错误。接下来是甘特图、时间线、用户旅程、看板和思维导图。请参阅[开发路线图](docs/ROADMAP.md)。

| 图表类型 | 状态 |
|---|---|
| 饼图（`pie`） | 已支持：`showData`、`title`、`accTitle`、`accDescr`，以及 `pie` 配置项 `textPosition`、`donutHole` 和 `legendPosition` |
| 流程图（`flowchart`、`graph`） | 已支持：所有节点形状（含 `@{ shape: … }`）、各种连线及长度、连线 ID、可自定义方向的子图、`classDef`、`class`、`style`、`linkStyle`、`click` 链接与提示文字、Markdown 标签，以及 `flowchart` 配置项 `curve`、`nodeSpacing`、`rankSpacing` 和 `wrappingWidth` |
| 时序图（`sequenceDiagram`） | 已支持：各种参与者和角色、各种箭头、激活条、注释、`loop`、`alt`、`opt`、`par`、`critical`、`break` 和 `rect` 块、分组框、`autonumber`、创建和销毁参与者，以及 `sequence` 配置项。参与者菜单（`link`、`links`）不会画成链接 |
| 状态图（`stateDiagram`、`stateDiagram-v2`） | 已支持：带描述的状态、可自定义方向的复合状态、并发区域、起点和终点、choice、fork 和 join、注释、`classDef`、`class`、`style` 和 `click` |
| 类图（`classDiagram`） | 已支持：含成员、泛型和注解的类、带标记和多重性的关系、命名空间、注释、棒棒糖接口、`classDef`、`cssClass`、`style`、`link`、`click` 和 `callback` |
| ER 图（`erDiagram`） | 已支持：含属性、键和注释的实体、鸦爪标记多重性的关系、别名、子图、`classDef`、`class` 和 `style` |
| 需求图（`requirementDiagram`） | 已支持：各种需求和元素及其字段、关系、`classDef`、`class` 和 `style` |
| 其他 | 计划中 |

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

`render` 返回 SVG 及其尺寸和链接，或返回一个 `Error`，其中包含错误代码（`Unsupported`、`Syntax`、`Semantic`、`Limit`）、行号和列号，以及指出问题位置的代码片段。`detect_type` 无需渲染即可判断图表类型。将 `max_width` 设为宿主可用的宽度，能画得更窄的图表就会变窄（饼图会把图例移到图的下方）。`click` 语句中的链接会经过清理，且只保留允许的 scheme（`link_schemes`，默认为 `http`、`https`、`mailto` 和 `tel`）；横向的流程图在放不进 `max_width` 时会改为纵向。每种图表类型都是一个 Cargo feature（`flowchart`、`pie`、…）；默认的 `all` feature 包含所有类型。

## 命令行

```sh
cargo install --path crates/fermaid-cli
fermaid diagram.mmd -o diagram.svg
cat diagram.mmd | fermaid --font-size 14 --max-width 360 --id-prefix fig1 > diagram.svg
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

可用的变量：`--fermaid-bg`、`--fermaid-fg`、`--fermaid-muted`、`--fermaid-line`、`--fermaid-surface`、`--fermaid-surface-alt`、`--fermaid-border`、`--fermaid-accent`、`--fermaid-radius`、`--fermaid-font`、`--fermaid-font-mono`，以及用于饼图扇区等数据系列颜色的 `--fermaid-series-1` 到 `--fermaid-series-8`。

## 开发

```sh
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all
cargo +nightly fuzz run render    # 需要 cargo-fuzz；另有 `pie`、`flowchart`、`sequence`、`state`、`class`、`er` 和 `requirement` 目标
cargo xtask compat pie --pele ../pele   # 从 pele 源码重新生成 Mermaid 规范测试用例
node tools/oracle/golden.mjs ../pele flowchart   # 重新生成 pele 对这些用例的解析结果
node tools/oracle/diff.mjs ../pele   # 与 Mermaid 自身的解析器做差异模糊测试
```

最低支持的 Rust 版本为 1.85。

## 致谢

fermaid 的算法、文本测量数据与限制移植自 Obsidian 团队 kepano 的 [pele](https://github.com/obsidianmd/pele)（MIT）。图表语法以及用作测试的解析器规范来自 [Mermaid](https://github.com/mermaid-js/mermaid)（MIT）。详见 [NOTICE](NOTICE)。

## 许可证

[MIT](LICENSE)
