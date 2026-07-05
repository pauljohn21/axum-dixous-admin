# dioxus-antd: Ant Design 组件库实现方案

## Context

当前 web crate 的 8 个 CRUD 页面存在大量重复 UI 代码：每个页面手动实现表单项(`div.form-control > label > input`)、弹窗(`dialog.modal-open`)、表格(`table.table-zebra`)、分页器(`div.join`)等，每个组件重复 3-4 次，累计产生约 500+ 行样板代码。需要封装一套完整的 Ant Design 组件库，提供声明式 API 消除重复，同时全量覆盖 Ant Design 60+ 组件。

**用户决策：**
- 独立 workspace crate（类似 dioxus-ant-icons）
- 全量覆盖 Ant Design 所有组件
- 使用 Ant Design CSS 原生样式（打包 antd.css）
- **不需要兼容 DaisyUI**，这是独立库，后续完全替换 DaisyUI

## 1. Crate 结构

创建 `/dioxus-antd/` 独立 crate，web 通过 `path = "../dioxus-antd"` 引用。

```
dioxus-antd/
├── Cargo.toml
├── assets/
│   ├── antd.css              # 亮色主题（由脚本生成，提交到 git）
│   └── antd.dark.css         # 暗色主题
├── scripts/
│   └── gen_antd_css.mjs      # Node.js 脚本：从 antd npm 包提取静态 CSS
└── src/
    ├── lib.rs                # 入口：re-export + AntdProvider
    ├── provider.rs           # AntdProvider 组件 + 主题 context
    ├── theme.rs              # Theme 枚举 (Light/Dark)
    ├── _utils.rs             # 内部工具 (class 合并等)
    ├── js/                   # JS 互操作（仅 web feature）
    │   ├── mod.rs
    │   ├── dom.rs            # focus trap, scroll lock
    │   └── position.rs       # 浮层定位计算
    ├── general/              # 通用 (4): Button, FloatButton, Icon, Typography
    ├── layout/               # 布局 (7): Divider, Flex, Grid, Layout, Masonry, Space, Splitter
    ├── navigation/           # 导航 (7): Anchor, Breadcrumb, Dropdown, Menu, Pagination, Steps, Tabs
    ├── data_entry/           # 数据录入 (18): AutoComplete~Upload
    ├── data_display/         # 数据展示 (20): Avatar~Tree
    └── feedback/             # 反馈 (11+): Alert~Watermark
```

## 2. CSS 策略

### 2.1 生成 antd.css
- Ant Design v5+ 使用 CSS-in-JS，无预编译 CSS
- 使用 `@ant-design/static-style-extract` 工具预烘焙出静态 CSS
- 脚本 `scripts/gen_antd_css.mjs` 执行一次，生成的 CSS 提交到 git（消费者不需要 Node.js）
- 使用 `hashPriority="high"` 模式生成（去掉 `:where()` 包裹，保持选择器优先级正常）

### 2.2 AntdProvider
- 在 App 根组件放置一次
- 根据主题加载 `antd.css` 或 `antd.dark.css`
- 同时加载 `AntIconStylesheet`（内部引用 dioxus-ant-icons）
- 提供 Theme context 供子组件读取
- **不需要 CSS 作用域隔离**，antd.css 直接全局生效，最终替换掉 DaisyUI

## 3. 组件实现模板

每个组件遵循统一模式：

```rust
// 1. 枚举类型映射 CSS class
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ButtonType { #[default] Default, Primary, Dashed, Text, Link }

impl ButtonType {
    fn class(&self) -> &'static str { match self { Self::Primary => "ant-btn-primary", ... } }
}

// 2. Props 对齐 Ant Design React API
#[component]
pub fn Button(
    button_type: Option<ButtonType>,  // "type" 是 Rust 保留字
    size: Option<ButtonSize>,
    danger: Option<bool>,
    loading: Option<bool>,
    onclick: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    // 3. 组装 CSS class，渲染 HTML 结构严格对应 Ant Design DOM
    rsx! { button { class: "{class_str}", ... {children} } }
}
```

关键约束：HTML 结构和 class 名严格对应 Ant Design 的真实 DOM，这样 antd.css 就能直接生效。

## 4. 核心组件设计

### Form + FormItem
- Form 提供 `Signal<FormInstance>` context，存储 values/errors
- FormItem 从 context 读写字段值，自动显示验证错误
- 替代当前 `div.form-control > label > input`（8行→3行）

### Table
- 声明式 API：`columns: Vec<TableColumn<T>>` + `data_source: Vec<T>`
- 支持 render 自定义列、排序、行选择
- 替代当前 `table.table-zebra` 手写 thead/tbody（40行→10行）

### Modal
- 声明式：`open` + `on_close` + `title` + `footer`
- CSS class 使用 `ant-modal-root > ant-modal-mask + ant-modal-wrap > ant-modal`
- 替代当前 `dialog.modal-open`（25行→8行）

### Pagination
- Props: `current`, `total`, `page_size`, `on_change`
- 替代当前 `div.join` 手动3按钮（15行→3行）

## 5. JS 互操作

仅 `web` feature 下编译，通过 `web-sys` 实现：

| 能力 | 用途 | 实现 |
|------|------|------|
| `get_bounding_client_rect()` | Dropdown/Tooltip/Select 定位 | js/position.rs |
| focus trap | Modal 键盘焦点循环 | js/dom.rs |
| body scroll lock | Modal/Drawer 打开时禁滚动 | js/dom.rs |
| `scroll_height` 读取 | Collapse/Menu 展开动画 | CSS max-height 过渡 |

## 6. 实现顺序（4 批次）

**第一批（核心 CRUD，消除最痛的重复代码）：**
AntdProvider, Theme, Button, Input/TextArea/Password/Search, Modal, Table, Form/FormItem, Pagination, Alert, Select, Space, Typography, Divider

**第二批（高频交互）：**
Tabs, Menu, Dropdown, Popconfirm, Switch, Checkbox, Radio, Tooltip, Tag, Badge, Spin, Empty, Tree, TreeSelect, Card, Popover, Breadcrumb, Steps

**第三批（完整 UI 体系）：**
Layout/Sider/Header/Content/Footer, Grid, Flex, FloatButton, Collapse, Descriptions, List, Avatar, Timeline, Statistic, Progress, Skeleton, Result, Drawer, Message, Notification, Anchor, InputNumber, Slider, Rate, Upload, AutoComplete

**第四批（低频高级）：**
DatePicker, TimePicker, Calendar, Cascader, Transfer, ColorPicker, Mentions, Carousel, QRCode, Segmented, Image, Watermark, Tour, Masonry, Splitter

## 7. 集成方式

### web/Cargo.toml 添加依赖
```toml
dioxus-antd = { path = "../dioxus-antd" }
```

### main.rs 修改
```rust
use dioxus_antd::AntdProvider;
use dioxus_antd::Theme;

fn App() -> Element {
    rsx! {
        AntdProvider { theme: Theme::Dark,
            Router::<Route> {}
        }
    }
}
```
注意：引入 AntdProvider 后可移除 DaisyUI 相关依赖（daisyui plugin、tailwind @plugin 指令等），Tailwind 仅保留工具类功能。

## 8. 验证方式

1. 每个组件实现后确保 `cargo check` 通过
2. 第一批完成后，改造 UserMgmt 页面作为集成验证
3. 启动 `dx serve` 在浏览器中验证组件渲染和交互
4. 全量完成后，将所有页面从 DaisyUI 迁移到 dioxus-antd，移除 DaisyUI 依赖

## 关键文件

- [dioxus-ant-icons/src/lib.rs](file:///Users/pauljohn/rust/axum-dixous-admin/dioxus-ant-icons/src/lib.rs) -- 参考 AntIconStylesheet 和 asset! 宏模式
- [web/src/pages/user.rs](file:///Users/pauljohn/rust/axum-dixous-admin/web/src/pages/user.rs) -- 第一批组件的验收标准页面
- [web/src/pages/role.rs](file:///Users/pauljohn/rust/axum-dixous-admin/web/src/pages/role.rs) -- 第二批 Tree/Tabs/Checkbox 的验收标准
- [web/src/main.rs](file:///Users/pauljohn/rust/axum-dixous-admin/web/src/main.rs) -- 需集成 AntdProvider
