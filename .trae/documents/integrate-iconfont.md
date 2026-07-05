# 集成 Iconfont 图标字体到前端

## Context

前端侧边栏当前使用 emoji 映射图标（📊、👤、📖 等），视觉效果差且与 daisyUI 主题不协调。用户希望使用阿里 Iconfont 图标字体库（项目名 Kitchen 3）来替换 emoji，实现专业的矢量图标显示。

## 实现步骤

### 1. 创建 iconfont 本地资源目录

在 `web/assets/iconfont/` 下放置从 iconfont.cn 下载的文件：
- `iconfont.css` — 图标字体样式（含 `@font-face` 和 `.iconfont` 类）
- `iconfont.ttf` / `iconfont.woff2` — 字体文件

> **用户操作**：前往 iconfont.cn → 项目 "Kitchen 3" → 下载至本地 → 将 `iconfont.css` 和字体文件放入 `web/assets/iconfont/`

### 2. 修改 iconfont.css 中的字体路径

将 `@font-face` 中的 `url` 改为相对路径（`./`），确保字体文件从同目录加载：
```css
src: url('./iconfont.woff2') format('woff2'),
     url('./iconfont.ttf') format('truetype');
```

### 3. 在 main.rs 中加载 iconfont.css

文件：`web/src/main.rs`

```rust
const ICONFONT_CSS: Asset = asset!("/assets/iconfont/iconfont.css");

// 在 App 组件的 RSX 中添加：
document::Link { rel: "stylesheet", href: ICONFONT_CSS }
```

### 4. 更新 sidebar.rs 的 render_icon 函数

文件：`web/src/components/sidebar.rs`

将 emoji 映射替换为 iconfont class：
```rust
fn render_icon(icon: &str) -> Element {
    rsx! {
        i { class: "iconfont icon-{icon}" }
    }
}
```

iconfont 的 class 命名规则为 `iconfont icon-{name}`，后端数据库的 icon 字段值需要与 iconfont 项目中的图标名一致。

### 5. 更新后端菜单迁移数据的 icon 字段

文件：`backend/server/data/migration/src/m20240422_075347_create_sys_menu.rs`

将 icon 值从通用名改为 iconfont 项目中的实际图标名。例如：
- `"odometer"` → `"icon-dashboard"` 或实际图标名
- `"user"` → `"icon-user"`
- `"dict"` → `"icon-dict"`
- 等等

具体名称取决于 Kitchen 3 项目中注册的图标名。

### 6. 在 tailwind.css 中添加 iconfont 样式调整（可选）

如果需要调整图标默认大小/颜色，可在 `assets/tailwind.css` 中追加：
```css
.iconfont {
    font-size: 18px;
}
```

## 涉及文件

| 文件 | 修改内容 |
|------|----------|
| `web/assets/iconfont/iconfont.css` | 新建 — iconfont 样式文件 |
| `web/assets/iconfont/iconfont.ttf` | 新建 — 字体文件 |
| `web/assets/iconfont/iconfont.woff2` | 新建 — 字体文件 |
| `web/src/main.rs` | 添加 `asset!` + `document::Link` |
| `web/src/components/sidebar.rs` | `render_icon` 改为 iconfont class |
| `backend/.../m20240422_075347_create_sys_menu.rs` | icon 字段值更新 |

## 验证

1. `cargo check -p web` 编译通过
2. 启动后端 `cargo run -p gateway`
3. 启动前端 `dx serve`
4. 浏览器打开页面，检查侧边栏图标是否正确显示为矢量图标
5. 浏览器 DevTools Network 确认 iconfont.css 和字体文件加载成功
