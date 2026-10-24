# fermaid

[English](README.md) | [繁體中文](README.zh-TW.md) | **简体中文** | [日本語](README.ja.md)

[![crates.io](https://img.shields.io/crates/v/fermaid.svg)](https://crates.io/crates/fermaid)
[![docs.rs](https://img.shields.io/docsrs/fermaid.svg)](https://docs.rs/fermaid)
[![license](https://img.shields.io/crates/l/fermaid.svg)](https://crates.io/crates/fermaid)

fermaid 以纯 Rust 将 [Mermaid](https://mermaid.js.org) 图表转换为 SVG，无需 JavaScript 运行时、无头浏览器或 Node.js。它移植自 [pele](https://github.com/obsidianmd/pele)（一个小巧、无依赖的 TypeScript 渲染器），支持 Mermaid 12.1.0 语法，并采用 pele 自己的布局与视觉风格。

> **状态：** alpha。已支持流程图、时序图、状态图、类图、ER 图、需求图、饼图、甘特图、桑基图、XY 图、象限图、时间线、用户旅程、看板和思维导图；其他图表类型目前会返回 `Unsupported` 错误。请参阅[开发路线图](docs/ROADMAP.md)。

| 图表类型 | 状态 |
|---|---|
| 饼图（`pie`） | 已支持：`showData`、`title`、`accTitle`、`accDescr`，以及 `pie` 配置项 `textPosition`、`donutHole` 和 `legendPosition` |
| 流程图（`flowchart`、`graph`） | 已支持：所有节点形状（含 `@{ shape: … }`）、各种连线及长度、连线 ID、可自定义方向的子图、`classDef`、`class`、`style`、`linkStyle`、`click` 链接与提示文字、Markdown 标签，以及 `flowchart` 配置项 `curve`、`nodeSpacing`、`rankSpacing` 和 `wrappingWidth` |
| 时序图（`sequenceDiagram`） | 已支持：各种参与者和角色、各种箭头、激活条、注释、`loop`、`alt`、`opt`、`par`、`critical`、`break` 和 `rect` 块、分组框、`autonumber`、创建和销毁参与者，以及 `sequence` 配置项。参与者菜单（`link`、`links`）不会画成链接 |
| 状态图（`stateDiagram`、`stateDiagram-v2`） | 已支持：带描述的状态、可自定义方向的复合状态、并发区域、起点和终点、choice、fork 和 join、注释、`classDef`、`class`、`style` 和 `click` |
| 类图（`classDiagram`） | 已支持：含成员、泛型和注解的类、带标记和多重性的关系、命名空间、注释、棒棒糖接口、`classDef`、`cssClass`、`style`、`link`、`click` 和 `callback` |
| ER 图（`erDiagram`） | 已支持：含属性、键和注释的实体、鸦爪标记多重性的关系、别名、子图、`classDef`、`class` 和 `style` |
| 需求图（`requirementDiagram`） | 已支持：各种需求和元素及其字段、关系、`classDef`、`class` 和 `style` |
| 甘特图（`gantt`） | 已支持：带日期、工期、`after` 和 `until` 的任务，分区，`done`、`active`、`crit`、`milestone` 和 `vert`，`excludes`、`includes` 和 `weekend`，`dateFormat`、`axisFormat`、`tickInterval`、`weekday`、`todayMarker`、`topAxis`，紧凑模式和 `click` 链接。日期一律为 UTC，“今天”由 `RenderOptions::now` 决定 |
| 时间线（`timeline`） | 已支持：标题、分区、时期及其事件 |
| 用户旅程（`journey`） | 已支持：标题、分区、带分数和参与者的任务 |
| 看板（`kanban`） | 已支持：列和卡片（含形状）、`@{ … }` 元数据（`assigned`、`ticket`、`priority`、`label`）、class，以及通过 `ticketBaseUrl` 的工单链接 |
| 思维导图（`mindmap`） | 已支持：所有节点形状、图标、class 和分支颜色；横向排列，宿主较窄时改为纵向 |
| 桑基图（`sankey`、`sankey-beta`） | 已支持：Mermaid 的 CSV 形式，每行一组 `source,target,value`，字段可加引号；以及 `sankey` 的配置键 `width`、`height`、`nodeWidth`、`nodePadding`、`nodeAlignment`、`nodeColors`、`linkColor`、`showValues`、`prefix`、`suffix` 和 `labelStyle` |
| XY 图（`xychart`、`xychart-beta`） | 已支持：`title`、以类别或范围表示的 `x-axis`、以范围表示的 `y-axis`、可带点标签的 `line` 与 `bar` 系列、`accTitle` 与 `accDescr`、`horizontal` 与 `vertical` 方向；以及 `xyChart` 的配置键 `width`、`height`、`chartOrientation`、`showTitle`、`showLegend`、`showDataLabel`、`xAxis.showTitle`、`xAxis.showTick`、`xAxis.showLabel`、`xAxis.showAxisLine`，以及 `yAxis` 下同样的四个 |
| 象限图（`quadrantChart`） | 已支持：`title`、以 `-->` 连接的 `x-axis` 与 `y-axis`、`quadrant-1` 到 `quadrant-4`、可带 `radius`、`color`、`stroke-color` 和 `stroke-width` 的点、`classDef` 与点的类别、`accTitle` 与 `accDescr`；以及 `quadrantChart` 的配置键 `chartWidth`、`chartHeight`、`titlePadding`、`xAxisLabelPadding`、`yAxisLabelPadding`、`xAxisPosition`、`yAxisPosition`、`quadrantPadding`、`quadrantTextTopPadding`、`pointRadius` 和 `pointTextPadding` |
| Git 图（`gitGraph`） | 已支持：`title`、`accTitle` 与 `accDescr`、可带 `id`、`msg`、`tag` 和 `NORMAL`、`REVERSE` 或 `HIGHLIGHT` 类型的 `commit`、可带 `order` 的 `branch`、`checkout` 与 `switch`、`merge` 和可带 `parent` 的 `cherry-pick`；以及 `gitGraph` 的配置键 `mainBranchName`、`mainBranchOrder`、`showCommitLabel`、`showBranches`、`rotateCommitLabel` 和 `parallelCommits` |
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

`render` 返回 SVG 及其尺寸和链接，或返回一个 `Error`，其中包含错误代码（`Unsupported`、`Syntax`、`Semantic`、`Limit`）、行号和列号，以及指出问题位置的代码片段。`detect_type` 无需渲染即可判断图表类型。将 `max_width` 设为宿主可用的宽度，能画得更窄的图表就会变窄（饼图会把图例移到图的下方）。`click` 语句中的链接会经过清理，且只保留允许的 scheme（`link_schemes`，默认为 `http`、`https`、`mailto` 和 `tel`）；横向的流程图在放不进 `max_width` 时会改为纵向。每种图表类型都是一个 Cargo feature（`flowchart`、`pie`、…）；默认的 `all` feature 包含所有类型。甘特图的“今天”取自 `now`（自 1970-01-01 UTC 起的毫秒数）；默认为 1970-01-01，使输出永远不受时钟影响。

## 命令行

```sh
cargo install fermaid-cli                # 从 crates.io 安装
cargo install --path crates/fermaid-cli  # 从源码安装
fermaid diagram.mmd -o diagram.svg
cat diagram.mmd | fermaid --font-size 14 --max-width 360 --id-prefix fig1 > diagram.svg
fermaid plan.mmd --now 1700000000000 -o plan.svg   # 画出今天标记的甘特图
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
cargo +nightly fuzz run render    # 需要 cargo-fuzz；另有 `pie`、`flowchart`、`sequence`、`state`、`class`、`er`、`requirement`、`timeline`、`journey`、`kanban`、`mindmap`、`gantt`、`sankey`、`xychart`、`quadrantchart` 和 `git` 目标
cargo xtask compat pie --pele ../pele   # 从 pele 源码重新生成 Mermaid 规范测试用例
node tools/oracle/golden.mjs ../pele flowchart   # 重新生成 pele 对这些用例的解析结果
node tools/oracle/diff.mjs ../pele   # 与 Mermaid 自身的解析器做差异模糊测试
```

最低支持的 Rust 版本为 1.85。

## 致谢

fermaid 的算法、文本测量数据与限制移植自 Obsidian 团队 kepano 的 [pele](https://github.com/obsidianmd/pele)（MIT）。图表语法以及用作测试的解析器规范来自 [Mermaid](https://github.com/mermaid-js/mermaid)（MIT）。详见 [NOTICE](NOTICE)。

## 许可证

可任选以下任一许可证使用：
- [MIT](LICENSE-MIT)
- [Apache-2.0](LICENSE-APACHE)
- [BSD-3-Clause](LICENSE-BSD3)
- [0BSD](LICENSE-0BSD)

详见 [LICENSE](LICENSE) 与 [NOTICE](NOTICE)。
