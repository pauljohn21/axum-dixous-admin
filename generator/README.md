# axum-admin-generator — 全栈 CRUD 代码生成器

> axum-dixous-admin 专用代码生成器，从 YAML 配置一键生成后端 (Migration + DAO + DTO + Service + API) 和前端 (Model + API + Component + i18n) 全栈代码。

## 快速开始

```bash
# 1. 初始化配置模板
cargo run --manifest-path generator/Cargo.toml -- init -n product

# 2. 编辑 product.yaml 修改字段定义

# 3. 预览将生成的代码
cargo run --manifest-path generator/Cargo.toml -- preview -c product.yaml

# 4. 生成全栈代码
cargo run --manifest-path generator/Cargo.toml -- generate -c product.yaml

# 5. 列出已生成的模块
cargo run --manifest-path generator/Cargo.toml -- list
```

## 命令说明

| 命令 | 说明 |
|------|------|
| `generate -c <file>` | 从 YAML 配置生成全栈 CRUD 模块 |
| `preview -c <file>` | 预览将生成的代码 (不写入文件) |
| `list` | 列出项目中已生成的模块 |
| `init -n <name>` | 初始化一个新的配置文件模板 |

### `generate` 命令选项

| 选项 | 说明 |
|------|------|
| `-c, --config <file>` | YAML 配置文件路径 (必填) |
| `--dry-run` | 预览模式，不写入文件 |
| `--skip-register` | 跳过注册步骤 (不修改 lib.rs/mod.rs) |
| `--backend-only` | 仅生成后端代码 |
| `--frontend-only` | 仅生成前端代码 |

## YAML 配置格式

```yaml
# 基础配置
table_name: sys_products        # 数据库表名 (sys_ 前缀)
resource: product                # 资源名 (snake_case 单数，用于 API 路径)
module_cn: "产品管理"             # 中文模块名
icon: goods                      # Element Plus 图标名

# 可选配置
generate_backend: true           # 是否生成后端 (默认 true)
generate_frontend: true          # 是否生成前端 (默认 true)
batch_delete: true               # 是否支持批量删除 (默认 true)

# 字段定义
fields:
  - name: name                   # 字段名 (snake_case)
    type: string                 # 字段类型
    nullable: true               # 是否可空 (默认 true)
    comment: "产品名称"            # 注释
    search: true                 # 简化搜索 (等同 search_type: like)
    # search_type: like          # 精确控制搜索方式
    # require: false             # 是否必填
    # default_value: ""          # 默认值
    # form: true                 # 表单中显示 (默认 true)
    # table: true                # 表格中显示 (默认 true)
    # desc: true                 # 详情中显示 (默认 true)
    # sort: false                # 可排序 (默认 false)
    # primary_key: false         # 主键 (默认 false)
    # enum_values: "a,b,c"       # 枚举值 (仅 type=enum)
```

## 支持的字段类型

| 类型 | 后端 Rust 类型 | SeaORM 列类型 | 前端 Rust 类型 | 说明 |
|------|---------------|---------------|----------------|------|
| `string` | `String` | `string` | `String` | 字符串 |
| `text` | `String` | `text` | `String` | 长文本 |
| `i8` | `i8` | `tiny_integer` | `i32` | 8位整数 |
| `i16` | `i16` | `small_integer` | `i32` | 16位整数 |
| `i32` | `i32` | `integer` | `i32` | 32位整数 |
| `i64` | `i64` | `big_integer` | `i32` | 64位整数 |
| `u64` | `u64` | `big_unsigned` | `i32` | 无符号64位整数 |
| `f32` | `f32` | `float` | `f32` | 单精度浮点 |
| `f64` | `f64` | `double` | `f64` | 双精度浮点 |
| `bool` | `bool` | `boolean` | `bool` | 布尔值 |
| `decimal` | `String` | `decimal_len(10,2)` | `String` | 十进制数 |
| `date` | `String` | `date` | `String` | 日期 |
| `datetime` | `String` | `date_time` | `String` | 日期时间 |
| `json` | `serde_json::Value` | `json` | `serde_json::Value` | JSON |
| `array` | `Vec<String>` | `json` | `Vec<String>` | 数组 |
| `enum` | `String` | `string` | `String` | 枚举 (配合 enum_values) |

## 支持的搜索类型

| 搜索类型 | 说明 | 生成的查询条件 |
|----------|------|----------------|
| `like` | 模糊匹配 | `LIKE '%keyword%'` |
| `eq` | 精确匹配 | `= value` |
| `ne` | 不等于 | `!= value` |
| `gt` | 大于 | `> value` |
| `lt` | 小于 | `< value` |
| `gte` | 大于等于 | `>= value` |
| `lte` | 小于等于 | `<= value` |
| `between` | 范围 | `BETWEEN start AND end` |

## 生成的文件清单

### 后端 (5 个文件 + 6 处注册)

| 文件 | 路径 |
|------|------|
| Migration | `backend/server/data/migration/src/m{timestamp}_create_{table}.rs` |
| DAO | `backend/server/data/model/src/dao/{table}.rs` |
| DTO | `backend/server/data/model/src/dto/sys_{resource}_dto.rs` |
| Service | `backend/server/service/src/sys_{resource}_service.rs` |
| API | `backend/server/api/src/{resource}_api.rs` |

注册修改:
- `migration/src/lib.rs` — 添加 mod 声明和 Migration 条目
- `dao/mod.rs` + `dao/prelude.rs` — 添加 mod 声明和 Entity 别名
- `dto/mod.rs` — 添加 mod 声明
- `service/lib.rs` — 添加 mod 声明
- `api/lib.rs` — 添加 mod 声明、paths、schemas、tags、routes merge

### 前端 (3 个文件 + 6 处注册)

| 文件 | 路径 |
|------|------|
| Model | `web/src/models/{resource}.rs` |
| API | `web/src/api/{resource}.rs` |
| Component | `web/src/components/{resource}_manage.rs` |

注册修改:
- `models/mod.rs` — 添加 mod 声明
- `api/mod.rs` — 添加 mod 声明
- `components/mod.rs` — 添加 mod 声明
- `router/mod.rs` — 添加 import 和路由变体
- `components/menu_item.rs` — 添加路由映射
- `i18n/mod.rs` — 添加 TKey 变体和翻译

## 完整示例

```yaml
table_name: sys_orders
resource: order
module_cn: "订单管理"
icon: shopping_cart
batch_delete: true

fields:
  - name: order_no
    type: string
    nullable: false
    comment: "订单号"
    require: true
    search: true
    sort: true

  - name: amount
    type: decimal
    nullable: false
    comment: "金额"
    require: true
    search_type: gte

  - name: status
    type: enum
    nullable: false
    comment: "状态"
    enum_values: "pending,paid,shipped,completed,cancelled"
    search_type: eq
    sort: true

  - name: remark
    type: text
    nullable: true
    comment: "备注"
    form: true
    table: false

  - name: created_by
    type: string
    nullable: true
    comment: "创建人"
    form: false
    table: true
```

## 生成后步骤

1. **检查迁移文件** — 确认表结构正确
2. **运行后端** — `cd backend && cargo run` (自动执行迁移)
3. **编译检查** — `cd backend && cargo check` / `cd web && cargo check`
4. **添加菜单** — 在数据库 `sys_base_menus` 表中添加菜单记录
5. **添加权限** — 在 Casbin 中添加 API 权限策略

## 命名规范

| 概念 | 示例 | 说明 |
|------|------|------|
| 表名 (复数) | `sys_products` | 数据库表名 |
| 资源名 (单数) | `product` | API 路径 `/api/product` |
| DAO 文件 | `sys_products.rs` | 与表名一致 |
| DTO 文件 | `sys_product_dto.rs` | `sys_` + 资源名 + `_dto` |
| Service 文件 | `sys_product_service.rs` | `sys_` + 资源名 + `_service` |
| API 文件 | `product_api.rs` | 资源名 + `_api` |
| 前端 Model | `product.rs` | 资源名 |
| 前端组件 | `product_manage.rs` | 资源名 + `_manage` |
| Entity 名 | `SysProducts` | PascalCase(表名) |
| DTO 结构体 | `SysProductInsertDTO` | `Sys` + PascalCase(资源名) + `DTO` |
| 组件名 | `ProductManage` | PascalCase(资源名) + `Manage` |

## 参照项目

本生成器参照 [gin-vue-admin](https://github.com/flipped-aurora/gin-vue-admin) 的自动化代码功能设计，适配 axum-dixous-admin 技术栈:

| gin-vue-admin | axum-admin-generator |
|---------------|---------------------|
| Go + Gin | Rust + Axum |
| GORM | SeaORM |
| Vue 3 + Element Plus | Dioxus 0.7 + dioxus-element-plug |
| Web UI 表单配置 | YAML 配置文件 |
| Go text/template | Rust format! 宏 |
| AST 注入 | 字符串锚点注入 |
