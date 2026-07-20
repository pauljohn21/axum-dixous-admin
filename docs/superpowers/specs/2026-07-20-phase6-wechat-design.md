# Phase 6 — 微信小程序详细设计

> 日期：2026-07-20
> 状态：📋 设计完成，待实施
> 前置条件：Phase 1-5 已完成

## 1. 现状分析

### 1.1 已有功能

| 页面 | 功能 | 状态 |
|------|------|------|
| `login` | 账号密码登录 + 微信一键登录 | ✅ 完整 |
| `dashboard` | 仪表盘统计 + 系统信息 | ✅ 完整 |
| `users` | 用户列表（分页 + 搜索 + 下拉刷新 + 上拉加载） | ✅ 完整（只读） |
| `profile` | 个人信息 + 修改密码 + 微信绑定 + 退出登录 | ✅ 完整 |

### 1.2 缺失功能（对比 Web 端）

| Web 端功能 | 小程序状态 | 后端 API |
|-----------|----------|----------|
| 角色管理 | ❌ 缺失 | `/api/role`, `/api/role/list`, `/api/role/{id}` |
| 菜单管理 | ❌ 缺失 | `/api/menu`, `/api/menu/list`, `/api/menu/{id}` |
| 字典管理 | ❌ 缺失 | `/api/dictionary`, `/api/dictionary/list`, `/api/dictionary/{id}` |
| 暗色模式 | ❌ 缺失 | - |
| 用户启用/禁用 | ❌ 缺失 | `/api/user/{id}` PUT |

### 1.3 代码质量现状

- **请求封装**: 已统一（`request.ts` 对齐 Web 端 `http/mod.rs`）
- **TypeScript 类型**: 基本完善，但缺少 Role/Menu/Dictionary 类型
- **错误处理**: 已统一（`request.ts` 中 401 跳转 + try/catch + Toast）
- **样式**: 全部硬编码 hex 颜色，不支持暗色模式

## 2. 目标

1. **功能对齐**: 新增角色管理、菜单管理（只读树）、字典管理页面
2. **用户操作**: 用户列表支持启用/禁用操作
3. **暗色模式**: CSS 变量驱动，跟随系统或手动切换
4. **代码质量**: TypeScript 类型完善，类型定义集中管理
5. **UI 打磨**: 加载骨架屏、空状态优化、交互反馈

## 3. 功能对齐设计

### 3.1 角色管理页面 (`pages/roles/roles`)

**功能：**
- 角色列表（分页 + 搜索）
- 查看角色详情
- 新增/编辑角色（弹窗表单）
- 删除角色（确认弹窗）

**后端 API：**
```
GET  /api/role/list?page=1&page_size=10&keyword=   → 角色列表
POST /api/role                                       → 新增角色
GET  /api/role/{id}                                  → 角色详情
PUT  /api/role/{id}                                  → 更新角色
DELETE /api/role/{id}                                → 删除角色
```

**数据模型：**
```typescript
interface RoleItem {
  id: number
  name: string | null
  keyword: string | null
  desc: string | null
  created_at: string | null
  updated_at: string | null
}
```

### 3.2 菜单管理页面 (`pages/menus/menus`)

**功能：**
- 菜单列表（树形展示，只读）
- 查看菜单详情

**后端 API：**
```
GET  /api/menu/list   → 菜单列表（平铺，前端构建树）
GET  /api/menu/{id}   → 菜单详情
```

**数据模型：**
```typescript
interface MenuItem {
  id: number
  parent_id: number | null
  name: string | null
  path: string | null
  component: string | null
  icon: string | null
  sort: number | null
  enable: number | null
  created_at: string | null
  updated_at: string | null
  children?: MenuItem[]
}
```

**树形展示方案：** 由于小程序没有树组件，使用缩进 + 图标折叠方式展示层级关系。

### 3.3 字典管理页面 (`pages/dicts/dicts`)

**功能：**
- 字典列表（分页 + 搜索）
- 查看字典详情

**后端 API：**
```
GET  /api/dictionary/list?page=1&page_size=10&keyword=   → 字典列表
GET  /api/dictionary/{id}                                 → 字典详情
```

**数据模型：**
```typescript
interface DictItem {
  id: number
  name: string | null
  type: string | null
  status: number | null
  desc: string | null
  created_at: string | null
  updated_at: string | null
}
```

### 3.4 用户启用/禁用

在用户列表页添加操作按钮：
- 点击用户项 → 弹出操作菜单（启用/禁用）
- 调用 `PUT /api/user/{id}` 传 `{ enable: 0|1 }`

## 4. 暗色模式设计

### 4.1 方案

使用 CSS 变量 + `wx.setNavigationBarColor` 实现暗色模式。

**`app.wxss` 中定义 CSS 变量：**
```css
page {
  /* 亮色主题 (默认) */
  --bg-color: #f0f2f5;
  --card-bg: #ffffff;
  --text-primary: #303133;
  --text-secondary: #909399;
  --border-color: #ebeef5;
  --primary-color: #409eff;
  --nav-bg: #409eff;
  --input-bg: #f5f7fa;
  --input-border: #e4e7ed;
}

page.dark {
  /* 暗色主题 */
  --bg-color: #1a1a2e;
  --card-bg: #16213e;
  --text-primary: #e0e0e0;
  --text-secondary: #909090;
  --border-color: #2a2a4a;
  --primary-color: #409eff;
  --nav-bg: #16213e;
  --input-bg: #0f3460;
  --input-border: #2a2a4a;
}
```

**所有页面 WXSS 将硬编码颜色替换为 `var(--xxx)`。**

### 4.2 切换逻辑

- 在 `profile` 页面添加暗色模式切换开关
- 使用 `wx.setStorageSync('admin_theme', 'dark'|'light')` 持久化
- 每个页面 `onShow` 时读取主题并设置 `page` class
- 通过 `wx.setNavigationBarColor` 同步导航栏颜色

### 4.3 主题工具 (`utils/theme.ts`)

```typescript
export type Theme = 'light' | 'dark'

export function getTheme(): Theme {
  return (wx.getStorageSync(THEME_KEY) as Theme) || 'light'
}

export function setTheme(theme: Theme): void {
  wx.setStorageSync(THEME_KEY, theme)
}

export function applyTheme(theme: Theme): void {
  const navBg = theme === 'dark' ? '#16213e' : '#409eff'
  const navText = theme === 'dark' ? '#e0e0e0' : '#ffffff'
  wx.setNavigationBarColor({
    frontColor: navText,
    backgroundColor: navBg,
  })
}
```

## 5. UI 打磨设计

### 5.1 加载骨架屏

在 `dashboard` 和 `users` 页面添加骨架屏组件替代纯文字 "加载中..."。

**骨架屏组件 (`components/skeleton/skeleton`):**
- 卡片骨架：灰色块 + 动画闪烁
- 列表骨架：3-5 行灰色条

### 5.2 空状态优化

统一空状态展示：图标 + 文字 + 操作引导。

### 5.3 交互反馈

- 按钮点击添加 `hover-class`
- 列表项点击添加 `hover-class`
- 下拉刷新添加 `loading` 动画

## 6. 代码质量改进

### 6.1 类型定义集中管理

在 `utils/api.ts` 中补全所有类型定义，确保每个 API 返回都有对应 interface。

### 6.2 页面配置统一

每个页面 `.json` 添加：
```json
{
  "enablePullDownRefresh": true,
  "backgroundColor": "#f0f2f5"
}
```

## 7. 页面结构变更

### app.json 更新

```json
{
  "pages": [
    "pages/login/login",
    "pages/dashboard/dashboard",
    "pages/users/users",
    "pages/roles/roles",
    "pages/menus/menus",
    "pages/dicts/dicts",
    "pages/profile/profile"
  ],
  "tabBar": {
    "list": [
      { "pagePath": "pages/dashboard/dashboard", "text": "仪表盘" },
      { "pagePath": "pages/users/users", "text": "用户" },
      { "pagePath": "pages/roles/roles", "text": "角色" },
      { "pagePath": "pages/profile/profile", "text": "我的" }
    ]
  }
}
```

> 菜单管理和字典管理从 "我的" 页面入口进入（非 tabBar）。

## 8. 验收标准

- [ ] 角色管理：列表 + 搜索 + 新增/编辑/删除
- [ ] 菜单管理：树形列表（只读）+ 详情查看
- [ ] 字典管理：列表 + 搜索 + 详情查看
- [ ] 用户启用/禁用操作
- [ ] 暗色模式：CSS 变量驱动，可切换，持久化
- [ ] 所有页面颜色使用 CSS 变量
- [ ] 加载骨架屏
- [ ] TypeScript 类型完整
- [ ] 空状态统一优化
