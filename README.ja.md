# fermaid

[English](README.md) | [繁體中文](README.zh-TW.md) | [简体中文](README.zh-CN.md) | **日本語**

[![crates.io](https://img.shields.io/crates/v/fermaid.svg)](https://crates.io/crates/fermaid)
[![docs.rs](https://img.shields.io/docsrs/fermaid.svg)](https://docs.rs/fermaid)
[![license](https://img.shields.io/crates/l/fermaid.svg)](https://crates.io/crates/fermaid)

fermaid は [Mermaid](https://mermaid.js.org) のダイアグラムを純粋な Rust で SVG に変換します。JavaScript ランタイム、ヘッドレスブラウザ、Node.js は不要です。小さく依存関係のない TypeScript 製レンダラー [pele](https://github.com/obsidianmd/pele) の移植で、Mermaid 12.1.0 の構文を対象とし、pele 独自のレイアウトとビジュアルスタイルを採用しています。

> **ステータス:** alpha。フローチャート、シーケンス図、状態遷移図、クラス図、ER 図、要求図、円グラフ、ガントチャート、サンキー図、XY チャート、クアドラントチャート、タイムライン、ユーザージャーニー、カンバン、マインドマップに対応しています。それ以外の種類は現在 `Unsupported` エラーを返します。[ロードマップ](docs/ROADMAP.md)を参照してください。

| ダイアグラムの種類 | 状況 |
|---|---|
| 円グラフ（`pie`） | 対応済み：`showData`、`title`、`accTitle`、`accDescr`、および `pie` の設定キー `textPosition`、`donutHole`、`legendPosition` |
| フローチャート（`flowchart`、`graph`） | 対応済み：すべてのノード形状（`@{ shape: … }` を含む）、あらゆる種類と長さのエッジ、エッジ ID、独自の向きを持てるサブグラフ、`classDef`、`class`、`style`、`linkStyle`、`click` のリンクとツールチップ、Markdown ラベル、および `flowchart` の設定キー `curve`、`nodeSpacing`、`rankSpacing`、`wrappingWidth` |
| シーケンス図（`sequenceDiagram`） | 対応済み：あらゆる種類の参加者とアクター、あらゆる種類の矢印、アクティベーション、ノート、`loop`、`alt`、`opt`、`par`、`critical`、`break`、`rect` ブロック、ボックス、`autonumber`、参加者の生成と破棄、および `sequence` の設定キー。参加者メニュー（`link`、`links`）はリンクとして描かれません |
| 状態遷移図（`stateDiagram`、`stateDiagram-v2`） | 対応済み：説明付きの状態、独自の方向を持つ複合状態、並行領域、開始と終了、choice、fork と join、ノート、`classDef`、`class`、`style`、`click` |
| クラス図（`classDiagram`） | 対応済み：メンバー、ジェネリック型、アノテーションを持つクラス、マーカーと多重度付きの関係、名前空間、ノート、ロリポップインターフェース、`classDef`、`cssClass`、`style`、`link`、`click`、`callback` |
| ER 図（`erDiagram`） | 対応済み：属性、キー、コメントを持つエンティティ、カラスの足記法の多重度付きリレーションシップ、エイリアス、サブグラフ、`classDef`、`class`、`style` |
| 要求図（`requirementDiagram`） | 対応済み：あらゆる種類の要求と要素およびそのフィールド、リレーションシップ、`classDef`、`class`、`style` |
| ガントチャート（`gantt`） | 対応済み：日付・期間・`after`・`until` を持つタスク、セクション、`done`・`active`・`crit`・`milestone`・`vert`、`excludes`・`includes`・`weekend`、`dateFormat`・`axisFormat`・`tickInterval`・`weekday`・`todayMarker`・`topAxis`、コンパクトモード、`click` リンク。日付はすべて UTC で、「今日」は `RenderOptions::now` で決まります |
| タイムライン（`timeline`） | 対応済み：タイトル、セクション、期間とそのイベント |
| ユーザージャーニー（`journey`） | 対応済み：タイトル、セクション、スコアとアクターを持つタスク |
| カンバン（`kanban`） | 対応済み：列と（形状付きの）カード、`@{ … }` メタデータ（`assigned`・`ticket`・`priority`・`label`）、クラス、`ticketBaseUrl` によるチケットリンク |
| マインドマップ（`mindmap`） | 対応済み：すべてのノード形状、アイコン、クラス、枝の色。横向きに配置し、ホストが狭いときは縦向きにします |
| サンキー図（`sankey`、`sankey-beta`） | 対応済み：Mermaid の CSV 形式。1 行につき `source,target,value` を 1 つ、フィールドは引用符で囲めます。`sankey` の設定キー `width`、`height`、`nodeWidth`、`nodePadding`、`nodeAlignment`、`nodeColors`、`linkColor`、`showValues`、`prefix`、`suffix`、`labelStyle` に対応 |
| XY チャート（`xychart`、`xychart-beta`） | 対応済み：`title`、カテゴリまたは範囲を持つ `x-axis`、範囲を持つ `y-axis`、点ラベルを持つ `line` と `bar` の系列、`accTitle` と `accDescr`、`horizontal` と `vertical` の向き。`xyChart` の設定キー `width`、`height`、`chartOrientation`、`showTitle`、`showLegend`、`showDataLabel`、`xAxis.showTitle`、`xAxis.showTick`、`xAxis.showLabel`、`xAxis.showAxisLine`、および `yAxis` 側の同じ 4 つのキーに対応 |
| クアドラントチャート（`quadrantChart`） | 対応済み：`title`、`-->` を持つ `x-axis` と `y-axis`、`quadrant-1` から `quadrant-4`、`radius`、`color`、`stroke-color`、`stroke-width` を持つ点、`classDef` と点のクラス、`accTitle` と `accDescr`。`quadrantChart` の設定キー `chartWidth`、`chartHeight`、`titlePadding`、`xAxisLabelPadding`、`yAxisLabelPadding`、`xAxisPosition`、`yAxisPosition`、`quadrantPadding`、`quadrantTextTopPadding`、`pointRadius`、`pointTextPadding` に対応 |
| Git グラフ（`gitGraph`） | 対応済み：`title`、`accTitle` と `accDescr`、`id`、`msg`、`tag`、`NORMAL`、`REVERSE`、`HIGHLIGHT` を持つ `commit`、`order` を持つ `branch`、`checkout` と `switch`、`merge` と `parent` を持つ `cherry-pick`。`gitGraph` の設定キー `mainBranchName`、`mainBranchOrder`、`showCommitLabel`、`showBranches`、`rotateCommitLabel`、`parallelCommits` に対応 |
| ブロック図（`block`、`block-beta`） | 対応済み：`columns` でグリッドに並べるブロック、`block:` と `end` による複合ブロック、`space`、Mermaid のすべての形状、方向を持つブロック矢印、ラベルと矢印の終端を持つリンク、`classDef`、`class` と `style`。`block` の設定キー `padding` に対応 |
| アーキテクチャ図（`architecture`、`architecture-beta`） | 対応済み：グループ化されたサービス、サービスの間のエッジとそのラベル、ジャンクション、アイコンを持つ `service`、`group` と `junction`。`architecture` の設定キー `iconSize`、`padding`、`fontSize` に対応。アイコンのフォントはないため、サービスの名前で図を表す |
| パケット図（`packet`、`packet-beta`） | 対応済み：`title`、`accTitle` と `accDescr`、ビット範囲を持つフィールドの行、`+bits` のフィールド。`packet` の設定キー `rowHeight`、`bitWidth`、`bitsPerRow`、`showBits`、`bitOrder`、`paddingX`、`paddingY` に対応 |
| レーダーチャート（`radar-beta`） | 対応済み：`title`、`accTitle` と `accDescr`、項目を持つ `axis` と `curve`、およびオプション `showLegend`、`ticks`、`min`、`max`、`graticule`。`radar` の設定キー `width`、`height`、`marginTop`、`marginRight`、`marginBottom`、`marginLeft`、`axisScaleFactor`、`axisLabelFactor`、`curveTension` に対応 |
| ツリーマップ（`treemap`） | 対応済み：セクションと葉から成る木とその値、`classDef` と `class` に対応 |
| C4 図（`C4Context`、`C4Container`、`C4Component`、`C4Dynamic`、`C4Deployment`） | 対応済み：`title`、`accTitle` と `accDescr`、人物、システム、コンテナ、コンポーネントと、その境界と関係、および `UpdateLayoutConfig` 文 |
| ユースケース図（`usecase-beta`） | 対応済み：`title`、`accTitle` と `accDescr`、アクターとユースケース、その関係。`usecase` の設定キー `nodeSpacing` と `rankSpacing` に対応 |
| インフォメーション図（`info`） | 対応済み：`title`、`accTitle` と `accDescr`、バージョンと本文の行 |
| ベン図（`venn-beta`） | 対応済み：各円とラベル。`venn` の設定キー `width`、`height`、`padding` に対応 |
| その他 | 予定 |

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

`render` は SVG とそのサイズ・リンクを返すか、エラーコード（`Unsupported`、`Syntax`、`Semantic`、`Limit`）、行番号と列番号、問題箇所を示すスニペットを含む `Error` を返します。`detect_type` はレンダリングせずにダイアグラムの種類を判定します。`max_width` にホストで使える幅を指定すると、幅を詰められるダイアグラムは狭く描かれます（円グラフは凡例をグラフの下に移します）。`click` 文のリンクはサニタイズされ、許可されたスキーム（`link_schemes`、デフォルトは `http`、`https`、`mailto`、`tel`）のものだけが残ります。横向きのフローチャートは `max_width` に収まらないときに縦向きで描かれます。ダイアグラムの種類ごとに Cargo feature（`flowchart`、`pie` など）があり、デフォルトの `all` feature にはすべての種類が含まれます。ガントチャートの「今日」は `now`（1970-01-01 UTC からのミリ秒）で決まります。デフォルトは 1970-01-01 で、出力が時計に左右されることはありません。

## コマンドライン

```sh
cargo install fermaid-cli                # crates.io からインストール
cargo install --path crates/fermaid-cli  # チェックアウトからインストール
fermaid diagram.mmd -o diagram.svg
cat diagram.mmd | fermaid --font-size 14 --max-width 360 --id-prefix fig1 > diagram.svg
fermaid plan.mmd --now 1700000000000 -o plan.svg   # 今日のマーカー付きのガントチャート
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

使用できる変数：`--fermaid-bg`、`--fermaid-fg`、`--fermaid-muted`、`--fermaid-line`、`--fermaid-surface`、`--fermaid-surface-alt`、`--fermaid-border`、`--fermaid-accent`、`--fermaid-radius`、`--fermaid-font`、`--fermaid-font-mono`、そして円グラフの扇形などデータ系列の色に使う `--fermaid-series-1` から `--fermaid-series-8`。

## 開発

```sh
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all
cargo +nightly fuzz run render    # cargo-fuzz が必要。`pie`、`flowchart`、`sequence`、`state`、`class`、`er`、`requirement`、`timeline`、`journey`、`kanban`、`mindmap`、`gantt`、`sankey`、`xychart`、`quadrantchart`、`git` ターゲットもあります
cargo xtask compat pie --pele ../pele   # pele のチェックアウトから Mermaid の仕様テストケースを再生成
node tools/oracle/golden.mjs ../pele flowchart   # それらに対する pele の解析結果を再生成
node tools/oracle/diff.mjs ../pele   # Mermaid 自身のパーサーとの差分ファジング
```

サポートする最小の Rust バージョンは 1.85 です。

## クレジット

fermaid のアルゴリズム、テキスト計測データ、制限は、Obsidian チームの kepano による [pele](https://github.com/obsidianmd/pele)（MIT）から移植しています。ダイアグラムの構文と、テストに使うパーサー仕様は [Mermaid](https://github.com/mermaid-js/mermaid)（MIT）に由来します。詳しくは [NOTICE](NOTICE) を参照してください。

## ライセンス

以下から選択して利用できます：
- [MIT](LICENSE-MIT)
- [Apache-2.0](LICENSE-APACHE)
- [BSD-3-Clause](LICENSE-BSD3)
- [0BSD](LICENSE-0BSD)

詳細は [LICENSE](LICENSE) および [NOTICE](NOTICE) を参照してください。
