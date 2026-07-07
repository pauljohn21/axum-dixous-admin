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
    },
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
        Commands::Generate { config, dry_run, skip_register } => {
            if let Err(e) = run_generate(&config, dry_run, skip_register) {
                eprintln!("\n❌ 生成失败: {}", e);
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
fn run_generate(config_path: &str, dry_run: bool, skip_register: bool) -> Result<(), String> {
    println!("📖 读取配置文件: {}", config_path);
    let config_str = std::fs::read_to_string(config_path)
        .map_err(|e| format!("读取配置文件失败: {}", e))?;
    let config: config::ModuleConfig = serde_yaml::from_str(&config_str)
        .map_err(|e| format!("解析配置文件失败: {}", e))?;

    let naming = naming::Naming::from_config(&config);
    let project_root = find_project_root()?;

    println!("🔧 模块信息:");
    println!("   表名:     {}", naming.table_name);
    println!("   资源名:   {}", naming.resource);
    println!("   模块名:   {}", naming.module_cn);
    println!("   实体名:   {}", naming.entity_name);
    println!("   组件名:   {}", naming.component_name);
    println!("   字段数:   {}", config.fields.len());
    println!();

    // === 生成后端文件 ===
    println!("📦 生成后端文件...");

    // 1. Migration
    let (migration_file, migration_module, migration_content) = backend_gen::gen_migration(&config, &naming);
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

    // === 生成前端文件 ===
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

    // === 注册模块 ===
    if !dry_run && !skip_register {
        println!("📝 注册模块...");

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

    println!();
    if dry_run {
        println!("✅ 预览完成 (dry-run 模式，未写入文件)");
    } else {
        println!("✅ 全栈 CRUD 模块生成完成!");
        println!();
        println!("📋 后续步骤:");
        println!("   1. 检查迁移文件并运行: cd backend && cargo run");
        println!("   2. 生成实体: cd backend/server/shell && sh gen_entity.sh");
        println!("   3. 后端编译检查: cd backend && cargo check");
        println!("   4. 前端编译检查: cd web && cargo check");
        println!("   5. 在数据库中添加菜单记录 (sys_base_menus 表)");
        println!("   6. 在 Casbin 中添加 API 权限策略");
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

# 字段定义
fields:
  - name: name
    type: string
    nullable: true
    comment: "名称"
    search: true              # 参与关键字搜索

  - name: description
    type: text
    nullable: true
    comment: "描述"
    search: true

  - name: status
    type: i8
    nullable: true
    comment: "状态"

  # 支持的类型: string, text, i8, i16, i32, i64, u64, f32, f64, bool, decimal, date, datetime
  # - name: price
  #   type: decimal
  #   nullable: true
  #   comment: "价格"
  #
  # - name: stock
  #   type: i32
  #   nullable: true
  #   comment: "库存"
  #
  # - name: enabled
  #   type: bool
  #   nullable: true
  #   comment: "是否启用"
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
