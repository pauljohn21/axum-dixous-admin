//! 全栈 CRUD 代码生成器
//!
//! 用法:
//! ```bash
//! # 从 YAML 配置生成模块
//! cargo run --manifest-path generator/Cargo.toml -- generate -c generator/examples/product.yaml
//!
//! # 预览将生成的文件 (不写入)
//! cargo run --manifest-path generator/Cargo.toml -- generate -c product.yaml --dry-run
//!
//! # 仅预览 (同 --dry-run)
//! cargo run --manifest-path generator/Cargo.toml -- preview -c product.yaml
//!
//! # 列出已生成的模块
//! cargo run --manifest-path generator/Cargo.toml -- list
//!
//! # 初始化新配置模板
//! cargo run --manifest-path generator/Cargo.toml -- init -n product
//! ```

mod config;
mod naming;
mod type_map;
mod backend_gen;
mod frontend_gen;
mod registry;

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "axum-admin-generator")]
#[command(about = "全栈 CRUD 代码生成器 — axum-dixous-admin 专用", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 从 YAML 配置文件生成全栈 CRUD 模块
    Generate {
        /// YAML 配置文件路径
        #[arg(short, long)]
        config: String,

        /// 预览模式 — 只打印将生成的文件，不写入
        #[arg(long)]
        dry_run: bool,

        /// 跳过注册步骤 (不修改 lib.rs/mod.rs 等现有文件)
        #[arg(long)]
        skip_register: bool,

        /// 仅生成后端代码
        #[arg(long)]
        backend_only: bool,

        /// 仅生成前端代码
        #[arg(long)]
        frontend_only: bool,
    },
    /// 预览将生成的文件 (不写入)
    Preview {
        /// YAML 配置文件路径
        #[arg(short, long)]
        config: String,
    },
    /// 列出项目中已生成的模块 (扫描 dao 目录)
    List,
    /// 初始化一个新的配置文件模板
    Init {
        /// 模块名称 (snake_case 单数，如 product)
        #[arg(short, long)]
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { config, dry_run, skip_register, backend_only, frontend_only } => {
            if let Err(e) = run_generate(&config, dry_run, skip_register, backend_only, frontend_only) {
                eprintln!("\n❌ 生成失败: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Preview { config } => {
            if let Err(e) = run_generate(&config, true, true, false, false) {
                eprintln!("\n❌ 预览失败: {}", e);
                std::process::exit(1);
            }
        }
        Commands::List => {
            if let Err(e) = run_list() {
                eprintln!("\n❌ 列出失败: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Init { name } => {
            if let Err(e) = run_init(&name) {
                eprintln!("\n❌ 初始化失败: {}", e);
                std::process::exit(1);
            }
        }
    }
}

/// 查找项目根目录 (包含 backend/ 和 web/ 的目录)
fn find_project_root() -> Result<PathBuf, String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;

    // 从当前目录向上查找包含 backend/ 和 web/ 的目录
    let mut current = cwd.as_path();
    loop {
        if current.join("backend").is_dir() && current.join("web").is_dir() {
            return Ok(current.to_path_buf());
        }
        match current.parent() {
            Some(parent) => current = parent,
            None => break,
        }
    }

    Err("无法找到项目根目录 (需包含 backend/ 和 web/ 目录)".into())
}

/// 执行生成命令
fn run_generate(
    config_path: &str,
    dry_run: bool,
    skip_register: bool,
    backend_only: bool,
    frontend_only: bool,
) -> Result<(), String> {
    println!("📖 读取配置文件: {}", config_path);
    let config_str = std::fs::read_to_string(config_path)
        .map_err(|e| format!("读取配置文件失败: {}", e))?;
    let config: config::ModuleConfig = serde_yaml::from_str(&config_str)
        .map_err(|e| format!("解析配置文件失败: {}", e))?;

    let naming = naming::Naming::from_config(&config);
    let project_root = find_project_root()?;

    // 确定生成范围
    let gen_backend = (config.generate_backend && !frontend_only) || backend_only;
    let gen_frontend = (config.generate_frontend && !backend_only) || frontend_only;

    println!("🔧 模块信息:");
    println!("   表名:     {}", naming.table_name);
    println!("   资源名:   {}", naming.resource);
    println!("   模块名:   {}", naming.module_cn);
    println!("   实体名:   {}", naming.entity_name);
    println!("   组件名:   {}", naming.component_name);
    println!("   字段数:   {}", config.fields.len());

    // 统计字段属性
    let searchable_count = config.fields.iter().filter(|f| f.is_searchable()).count();
    let sortable_count = config.fields.iter().filter(|f| f.sort).count();
    let form_count = config.fields.iter().filter(|f| f.form).count();
    let table_count = config.fields.iter().filter(|f| f.table).count();
    println!("   可搜索:   {} 个字段", searchable_count);
    println!("   可排序:   {} 个字段", sortable_count);
    println!("   表单显示: {} 个字段", form_count);
    println!("   表格显示: {} 个字段", table_count);
    println!("   批量删除: {}", if config.batch_delete { "是" } else { "否" });
    println!();

    // === 生成后端文件 ===
    let mut migration_module = String::new();
    if gen_backend {
        println!("📦 生成后端文件...");

        // 1. Migration
        let (migration_file, mig_module, migration_content) = backend_gen::gen_migration(&config, &naming);
        migration_module = mig_module;
        let migration_dir = project_root.join("backend/server/data/migration/src");
        let migration_path = migration_dir.join(&migration_file);
        write_file(&migration_path, &migration_content, dry_run)?;
        println!("   ✓ {}", migration_file);

        // 2. DAO
        let dao_content = backend_gen::gen_dao(&config, &naming);
        let dao_path = project_root.join(format!("backend/server/data/model/src/dao/{}.rs", naming.dao_file));
        write_file(&dao_path, &dao_content, dry_run)?;
        println!("   ✓ {}.rs", naming.dao_file);

        // 3. DTO
        let dto_content = backend_gen::gen_dto(&config, &naming);
        let dto_path = project_root.join(format!("backend/server/data/model/src/dto/{}.rs", naming.dto_file));
        write_file(&dto_path, &dto_content, dry_run)?;
        println!("   ✓ {}.rs", naming.dto_file);

        // 4. Service
        let service_content = backend_gen::gen_service(&config, &naming);
        let service_path = project_root.join(format!("backend/server/service/src/{}.rs", naming.service_file));
        write_file(&service_path, &service_content, dry_run)?;
        println!("   ✓ {}.rs", naming.service_file);

        // 5. API
        let api_content = backend_gen::gen_api(&config, &naming);
        let api_path = project_root.join(format!("backend/server/api/src/{}.rs", naming.api_file));
        write_file(&api_path, &api_content, dry_run)?;
        println!("   ✓ {}.rs", naming.api_file);
    }

    // === 生成前端文件 ===
    if gen_frontend {
        println!("🖥️  生成前端文件...");

        // 6. Model
        let model_content = frontend_gen::gen_model(&config, &naming);
        let model_path = project_root.join(format!("web/src/models/{}.rs", naming.fe_file));
        write_file(&model_path, &model_content, dry_run)?;
        println!("   ✓ {}.rs", naming.fe_file);

        // 7. API
        let fe_api_content = frontend_gen::gen_api(&config, &naming);
        let fe_api_path = project_root.join(format!("web/src/api/{}.rs", naming.fe_file));
        write_file(&fe_api_path, &fe_api_content, dry_run)?;
        println!("   ✓ {}.rs", naming.fe_file);

        // 8. Component
        let component_content = frontend_gen::gen_component(&config, &naming);
        let component_path = project_root.join(format!("web/src/components/{}.rs", naming.component_file));
        write_file(&component_path, &component_content, dry_run)?;
        println!("   ✓ {}.rs", naming.component_file);
    }

    // === 注册模块 ===
    if !dry_run && !skip_register {
        println!("📝 注册模块...");

        if gen_backend {
            // 后端注册
            let migration_lib = project_root.join("backend/server/data/migration/src/lib.rs");
            registry::register_migration(&migration_lib, &migration_module, &naming)?;
            println!("   ✓ migration/lib.rs");

            let dao_mod = project_root.join("backend/server/data/model/src/dao/mod.rs");
            let dao_prelude = project_root.join("backend/server/data/model/src/dao/prelude.rs");
            registry::register_dao(&dao_mod, &dao_prelude, &naming)?;
            println!("   ✓ dao/mod.rs + prelude.rs");

            let dto_mod = project_root.join("backend/server/data/model/src/dto/mod.rs");
            registry::register_dto(&dto_mod, &naming)?;
            println!("   ✓ dto/mod.rs");

            let service_lib = project_root.join("backend/server/service/src/lib.rs");
            registry::register_service(&service_lib, &naming)?;
            println!("   ✓ service/lib.rs");

            let api_lib = project_root.join("backend/server/api/src/lib.rs");
            registry::register_api(&api_lib, &config, &naming)?;
            println!("   ✓ api/lib.rs");
        }

        if gen_frontend {
            // 前端注册
            let fe_models_mod = project_root.join("web/src/models/mod.rs");
            registry::register_fe_model(&fe_models_mod, &naming)?;
            println!("   ✓ models/mod.rs");

            let fe_api_mod = project_root.join("web/src/api/mod.rs");
            registry::register_fe_api(&fe_api_mod, &naming)?;
            println!("   ✓ api/mod.rs");

            let fe_components_mod = project_root.join("web/src/components/mod.rs");
            registry::register_fe_component(&fe_components_mod, &naming)?;
            println!("   ✓ components/mod.rs");

            let fe_router = project_root.join("web/src/router/mod.rs");
            registry::register_fe_router(&fe_router, &naming)?;
            println!("   ✓ router/mod.rs");

            let fe_menu_item = project_root.join("web/src/components/menu_item.rs");
            registry::register_fe_menu_item(&fe_menu_item, &naming)?;
            println!("   ✓ menu_item.rs");

            let fe_i18n = project_root.join("web/src/i18n/mod.rs");
            registry::register_fe_i18n(&fe_i18n, &config, &naming)?;
            println!("   ✓ i18n/mod.rs");
        }
    }

    println!();
    if dry_run {
        println!("✅ 预览完成 (dry-run 模式，未写入文件)");
    } else {
        println!("✅ 全栈 CRUD 模块生成完成!");
        println!();
        println!("📋 后续步骤:");
        if gen_backend {
            println!("   1. 检查迁移文件并运行: cd backend && cargo run");
            println!("   2. 生成实体: cd backend/server/shell && sh gen_entity.sh");
            println!("   3. 后端编译检查: cd backend && cargo check");
        }
        if gen_frontend {
            if gen_backend {
                println!("   4. 前端编译检查: cd web && cargo check");
            } else {
                println!("   1. 前端编译检查: cd web && cargo check");
            }
        }
        println!("   ⚠️  在数据库中添加菜单记录 (sys_base_menus 表)");
        println!("   ⚠️  在 Casbin 中添加 API 权限策略");
    }

    Ok(())
}

/// 执行列出命令 — 扫描已生成的模块
fn run_list() -> Result<(), String> {
    let project_root = find_project_root()?;
    let dao_dir = project_root.join("backend/server/data/model/src/dao");

    println!("📋 已生成的后端模块 (DAO 目录):\n");

    if !dao_dir.is_dir() {
        println!("   (未找到 DAO 目录)");
        return Ok(());
    }

    let mut modules: Vec<(String, String)> = Vec::new();

    for entry in std::fs::read_dir(&dao_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "rs") {
            let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
            if file_name == "mod" || file_name == "prelude" {
                continue;
            }
            // 读取文件内容查找 table_name
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            let table_name = content
                .lines()
                .find_map(|line| {
                    if line.contains("table_name") {
                        let start = line.find('"')?;
                        let end = line.rfind('"')?;
                        if start < end {
                            Some(line[start + 1..end].to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| file_name.clone());

            modules.push((file_name, table_name));
        }
    }

    modules.sort();

    if modules.is_empty() {
        println!("   (暂无生成的模块)");
    } else {
        for (i, (file_name, table_name)) in modules.iter().enumerate() {
            println!("   {}. {} (表: {})", i + 1, file_name, table_name);
        }
    }

    // 也扫描前端组件
    let component_dir = project_root.join("web/src/components");
    if component_dir.is_dir() {
        println!("\n📋 已生成的前端组件:\n");
        let mut components: Vec<String> = Vec::new();
        for entry in std::fs::read_dir(&component_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "rs") {
                let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
                if file_name.ends_with("_manage") {
                    components.push(file_name);
                }
            }
        }
        components.sort();
        if components.is_empty() {
            println!("   (暂无生成的组件)");
        } else {
            for (i, comp) in components.iter().enumerate() {
                println!("   {}. {}", i + 1, comp);
            }
        }
    }

    Ok(())
}

/// 执行初始化命令
fn run_init(name: &str) -> Result<(), String> {
    let template = format!(
        r#"# {name} 模块配置
# 修改后运行: cargo run --manifest-path generator/Cargo.toml -- generate -c {name}.yaml

# 数据库表名 (sys_ 前缀)
table_name: sys_{name}s

# 资源名 (snake_case 单数，用于 API 路径)
resource: {name}

# 中文模块名
module_cn: "{name_pascal}管理"

# Element Plus 图标名 (参考: https://element-plus.org/zh-CN/component/icon.html)
icon: document

# 是否生成后端 (可选, 默认 true)
# generate_backend: true

# 是否生成前端 (可选, 默认 true)
# generate_frontend: true

# 是否支持批量删除 (可选, 默认 true)
# batch_delete: true

# 字段定义
fields:
  - name: name
    type: string
    nullable: true
    comment: "名称"
    search: true              # 简化搜索 (等同 search_type: like)
    # search_type: like       # 精确控制: like | eq | ne | gt | lt | gte | lte | between
    # require: false           # 是否必填
    # default_value: ""        # 默认值
    # form: true               # 是否在表单中显示 (默认 true)
    # table: true              # 是否在表格中显示 (默认 true)
    # sort: false              # 是否可排序

  - name: description
    type: text
    nullable: true
    comment: "描述"
    search: true

  - name: status
    type: i8
    nullable: true
    comment: "状态"
    # search_type: eq          # 精确匹配搜索

  # 支持的类型: string, text, i8, i16, i32, i64, u64, f32, f64, bool, decimal, date, datetime, json, array, enum
  #
  # - name: price
  #   type: decimal
  #   nullable: true
  #   comment: "价格"
  #
  # - name: stock
  #   type: i32
  #   nullable: true
  #   comment: "库存"
  #   sort: true               # 可排序
  #
  # - name: enabled
  #   type: bool
  #   nullable: true
  #   comment: "是否启用"
  #
  # - name: tags
  #   type: array
  #   nullable: true
  #   comment: "标签"
  #
  # - name: metadata
  #   type: json
  #   nullable: true
  #   comment: "元数据"
  #
  # - name: level
  #   type: enum
  #   nullable: true
  #   comment: "级别"
  #   enum_values: "low,medium,high"
"#,
        name = name,
        name_pascal = naming::to_pascal(name),
    );

    let file_name = format!("{}.yaml", name);
    std::fs::write(&file_name, &template)
        .map_err(|e| format!("写入配置文件失败: {}", e))?;

    println!("✅ 配置模板已创建: {}", file_name);
    println!();
    println!("📋 下一步:");
    println!("   1. 编辑 {} 修改字段定义", file_name);
    println!("   2. 运行: cargo run --manifest-path generator/Cargo.toml -- generate -c {}", file_name);

    Ok(())
}

/// 写入文件 (或预览)
fn write_file(path: &Path, content: &str, dry_run: bool) -> Result<(), String> {
    if dry_run {
        println!("\n--- {} ---", path.display());
        println!("{}", content);
        println!("--- end ---\n");
        return Ok(());
    }

    // 确保父目录存在
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("创建目录失败: {} - {}", parent.display(), e))?;
    }

    std::fs::write(path, content)
        .map_err(|e| format!("写入文件失败: {} - {}", path.display(), e))?;
    Ok(())
}
