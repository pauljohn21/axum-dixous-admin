//! 注册补丁模块
//!
//! 修改现有文件以注册新生成的模块:
//! - 后端: migration/lib.rs, dao/mod.rs, dao/prelude.rs, dto/mod.rs, service/lib.rs, api/lib.rs
//! - 前端: models/mod.rs, api/mod.rs, components/mod.rs, router/mod.rs, menu_item.rs, i18n/mod.rs

use crate::config::ModuleConfig;
use crate::naming::Naming;
use std::path::Path;

/// 注册结果
#[allow(dead_code)]
pub struct RegistryResult {
    pub warnings: Vec<String>,
}

/// 在文件中查找锚点并插入内容
fn insert_before_line(content: &str, anchor: &str, insertion: &str) -> String {
    if let Some(pos) = content.find(anchor) {
        let mut result = content[..pos].to_string();
        result.push_str(insertion);
        result.push_str(&content[pos..]);
        result
    } else {
        // 如果找不到锚点，追加到末尾
        let mut result = content.to_string();
        if !result.ends_with('\n') {
            result.push('\n');
        }
        result.push_str(insertion);
        result
    }
}

/// 在文件中查找锚点并在其后面插入内容
fn insert_after_line(content: &str, anchor: &str, insertion: &str) -> String {
    if let Some(pos) = content.find(anchor) {
        // 找到该行的末尾
        let line_end = content[pos..].find('\n').map(|i| pos + i + 1).unwrap_or(content.len());
        let mut result = content[..line_end].to_string();
        result.push_str(insertion);
        result.push_str(&content[line_end..]);
        result
    } else {
        let mut result = content.to_string();
        if !result.ends_with('\n') {
            result.push('\n');
        }
        result.push_str(insertion);
        result
    }
}

/// 在文件末尾追加内容 (确保前面有换行)
fn append_line(content: &str, line: &str) -> String {
    let mut result = content.trim_end().to_string();
    result.push('\n');
    result.push_str(line);
    result.push('\n');
    result
}

/// 注册后端迁移
pub fn register_migration(
    lib_rs_path: &Path,
    migration_module: &str,
    _naming: &Naming,
) -> Result<(), String> {
    let content = std::fs::read_to_string(lib_rs_path)
        .map_err(|e| format!("读取迁移 lib.rs 失败: {}", e))?;

    // 添加 mod 声明 (在最后一个 mod 声明之后)
    let mod_line = format!("mod {};\n", migration_module);
    if content.contains(&mod_line) {
        return Ok(()); // 已存在
    }

    // 找到最后一个 "mod m" 行并在其后插入
    let new_content = insert_after_last_match(&content, "mod m", &mod_line);

    // 添加到 migrations vec (在最后一个 Box::new 之后)
    let migration_entry = format!(
        "            Box::new({}::Migration),\n",
        migration_module
    );
    let new_content = insert_after_last_match(&new_content, "Box::new(m", &migration_entry);

    std::fs::write(lib_rs_path, new_content)
        .map_err(|e| format!("写入迁移 lib.rs 失败: {}", e))?;
    Ok(())
}

/// 在文件中找到最后一个匹配某模式的行，在其后插入
fn insert_after_last_match(content: &str, pattern: &str, insertion: &str) -> String {
    let last_pos = content.rfind(pattern);
    if let Some(pos) = last_pos {
        let line_end = content[pos..].find('\n').map(|i| pos + i + 1).unwrap_or(content.len());
        let mut result = content[..line_end].to_string();
        result.push_str(insertion);
        result.push_str(&content[line_end..]);
        result
    } else {
        let mut result = content.to_string();
        if !result.ends_with('\n') {
            result.push('\n');
        }
        result.push_str(insertion);
        result
    }
}

/// 注册 DAO
pub fn register_dao(
    dao_mod_path: &Path,
    prelude_path: &Path,
    naming: &Naming,
) -> Result<(), String> {
    // dao/mod.rs
    let mod_line = format!("pub mod {};\n", naming.dao_file);
    let content = std::fs::read_to_string(dao_mod_path)
        .map_err(|e| format!("读取 dao/mod.rs 失败: {}", e))?;
    if !content.contains(&mod_line) {
        let new_content = append_line(&content, &format!("pub mod {};", naming.dao_file));
        std::fs::write(dao_mod_path, new_content)
            .map_err(|e| format!("写入 dao/mod.rs 失败: {}", e))?;
    }

    // dao/prelude.rs
    let prelude_line = format!(
        "pub use super::{}::Entity as {};\n",
        naming.dao_file, naming.entity_name
    );
    let content = std::fs::read_to_string(prelude_path)
        .map_err(|e| format!("读取 dao/prelude.rs 失败: {}", e))?;
    if !content.contains(&prelude_line) {
        let new_content = append_line(
            &content,
            &format!("pub use super::{}::Entity as {};", naming.dao_file, naming.entity_name),
        );
        std::fs::write(prelude_path, new_content)
            .map_err(|e| format!("写入 dao/prelude.rs 失败: {}", e))?;
    }

    Ok(())
}

/// 注册 DTO
pub fn register_dto(dto_mod_path: &Path, naming: &Naming) -> Result<(), String> {
    let mod_line = format!("pub mod {};", naming.dto_file);
    let content = std::fs::read_to_string(dto_mod_path)
        .map_err(|e| format!("读取 dto/mod.rs 失败: {}", e))?;
    if !content.contains(&mod_line) {
        let new_content = append_line(&content, &mod_line);
        std::fs::write(dto_mod_path, new_content)
            .map_err(|e| format!("写入 dto/mod.rs 失败: {}", e))?;
    }
    Ok(())
}

/// 注册 Service
pub fn register_service(service_lib_path: &Path, naming: &Naming) -> Result<(), String> {
    let mod_line = format!("pub mod {};", naming.service_file);
    let content = std::fs::read_to_string(service_lib_path)
        .map_err(|e| format!("读取 service/lib.rs 失败: {}", e))?;
    if !content.contains(&mod_line) {
        let new_content = insert_before_line(
            &content,
            "/// 仪表盘统计数据",
            &format!("{}\n", mod_line),
        );
        std::fs::write(service_lib_path, new_content)
            .map_err(|e| format!("写入 service/lib.rs 失败: {}", e))?;
    }
    Ok(())
}

/// 注册 API (最复杂的注册)
pub fn register_api(
    api_lib_path: &Path,
    _config: &ModuleConfig,
    naming: &Naming,
) -> Result<(), String> {
    let content = std::fs::read_to_string(api_lib_path)
        .map_err(|e| format!("读取 api/lib.rs 失败: {}", e))?;

    let mut new_content = content;

    // 1. 添加 pub mod
    let mod_line = format!("pub mod {};", naming.api_file);
    if !new_content.contains(&mod_line) {
        // 在最后一个 "pub mod xxx_api;" 之后插入
        new_content = insert_after_last_match(&new_content, "pub mod ", &format!("{}\n", mod_line));
    }

    // 2. 添加 paths — 使用精确锚点：最后一行 operation_record_api
    let paths_line = format!(
        "        {}::create, {}::list, {}::get_by_id, {}::update, {}::{},",
        naming.api_file, naming.api_file, naming.api_file, naming.api_file,
        naming.api_file, format!("delete_{}", naming.resource)
    );
    if !new_content.contains(&paths_line) {
        new_content = insert_after_line(
            &new_content,
            "operation_record_api::create, operation_record_api::list, operation_record_api::get_by_id, operation_record_api::update, operation_record_api::delete_record",
            &format!("{}\n", paths_line),
        );
    }

    // 3. 添加 schemas
    let insert_dto = format!("{}InsertDTO", naming.pascal_singular);
    let update_dto = format!("{}UpdateDTO", naming.pascal_singular);
    let query_dto = format!("{}QueryDTO", naming.pascal_singular);
    let schema_line = format!(
        "        {}::{}, {}::{}, {}::{},",
        naming.dto_file, insert_dto,
        naming.dto_file, update_dto,
        naming.dto_file, query_dto,
    );
    if !new_content.contains(&schema_line) {
        // 使用精确锚点：在最后一个 DTO schema 行之后插入
        new_content = insert_after_line(
            &new_content,
            "sys_operation_record_dto::SysOperationRecordInsertDTO, sys_operation_record_dto::SysOperationRecordUpdateDTO, sys_operation_record_dto::SysOperationRecordQueryDTO,",
            &format!("{}\n", schema_line),
        );
    }

    // 添加 Model schema
    let model_schema_line = format!("        {}::Model,", naming.dao_file);
    if !new_content.contains(&model_schema_line) {
        // 在 "sys_operation_records::Model" 之后添加
        new_content = insert_after_line(
            &new_content,
            "sys_operation_records::Model",
            &format!("{}\n", model_schema_line),
        );
    }

    // 4. 添加 tag
    let tag_line = format!(
        "        (name = \"{}\", description = \"{} CRUD\"),",
        naming.module_cn, naming.module_cn
    );
    if !new_content.contains(&tag_line) {
        // 使用精确锚点：在最后一个 tag 行之后插入
        new_content = insert_after_line(
            &new_content,
            "(name = \"操作记录\", description = \"操作日志查询\")",
            &format!("{}\n", tag_line),
        );
    }

    // 5. 添加 routes merge
    let routes_line = format!("        .merge({}::routes())", naming.api_file);
    if !new_content.contains(&routes_line) {
        // 在最后一个 .merge 之后插入
        new_content = insert_after_last_match(
            &new_content,
            ".merge(",
            &format!("{}\n", routes_line),
        );
    }

    std::fs::write(api_lib_path, new_content)
        .map_err(|e| format!("写入 api/lib.rs 失败: {}", e))?;
    Ok(())
}

/// 在特定 section (如 paths(, schemas(, tags() 中找到最后一行匹配并在其后插入
#[allow(dead_code)]
fn insert_after_last_match_in_section(
    content: &str,
    section_start: &str,
    line_prefix: &str,
    _line_suffix: &str,
    insertion: &str,
) -> String {
    // 找到 section 开始位置
    let section_pos = match content.find(section_start) {
        Some(p) => p,
        None => {
            return format!("{}{}", content, insertion);
        }
    };

    // 从 section 开始往后找，直到遇到 "))" 或 section 结束
    let after_section = &content[section_pos..];
    let section_end = after_section.find("))").map(|i| section_pos + i).unwrap_or(content.len());

    let section_content = &content[section_pos..section_end];

    // 找到 section 中最后一个以 line_prefix 开头的行
    let last_match = section_content.rfind(line_prefix);
    if let Some(pos) = last_match {
        let abs_pos = section_pos + pos;
        let line_end = content[abs_pos..].find('\n').map(|i| abs_pos + i + 1).unwrap_or(content.len());
        let mut result = content[..line_end].to_string();
        result.push_str(insertion);
        result.push_str(&content[line_end..]);
        result
    } else {
        // 在 section_start 行之后插入
        let line_end = content[section_pos..].find('\n').map(|i| section_pos + i + 1).unwrap_or(content.len());
        let mut result = content[..line_end].to_string();
        result.push_str(insertion);
        result.push_str(&content[line_end..]);
        result
    }
}

/// 注册前端 model
pub fn register_fe_model(mod_path: &Path, naming: &Naming) -> Result<(), String> {
    let mod_line = format!("pub mod {};", naming.fe_file);
    let content = std::fs::read_to_string(mod_path)
        .map_err(|e| format!("读取 models/mod.rs 失败: {}", e))?;
    if !content.contains(&mod_line) {
        let new_content = append_line(&content, &mod_line);
        std::fs::write(mod_path, new_content)
            .map_err(|e| format!("写入 models/mod.rs 失败: {}", e))?;
    }
    Ok(())
}

/// 注册前端 api
pub fn register_fe_api(mod_path: &Path, naming: &Naming) -> Result<(), String> {
    let mod_line = format!("pub mod {};", naming.fe_file);
    let content = std::fs::read_to_string(mod_path)
        .map_err(|e| format!("读取 api/mod.rs 失败: {}", e))?;
    if !content.contains(&mod_line) {
        let new_content = append_line(&content, &mod_line);
        std::fs::write(mod_path, new_content)
            .map_err(|e| format!("写入 api/mod.rs 失败: {}", e))?;
    }
    Ok(())
}

/// 注册前端组件
pub fn register_fe_component(mod_path: &Path, naming: &Naming) -> Result<(), String> {
    let mod_line = format!("pub mod {};", naming.component_file);
    let content = std::fs::read_to_string(mod_path)
        .map_err(|e| format!("读取 components/mod.rs 失败: {}", e))?;
    if !content.contains(&mod_line) {
        let new_content = append_line(&content, &mod_line);
        std::fs::write(mod_path, new_content)
            .map_err(|e| format!("写入 components/mod.rs 失败: {}", e))?;
    }
    Ok(())
}

/// 注册前端路由
pub fn register_fe_router(router_path: &Path, naming: &Naming) -> Result<(), String> {
    let content = std::fs::read_to_string(router_path)
        .map_err(|e| format!("读取 router/mod.rs 失败: {}", e))?;

    let mut new_content = content;

    // 1. 添加 import
    let import_line = format!(
        "use crate::components::{}::{};",
        naming.component_file, naming.component_name
    );
    if !new_content.contains(&import_line) {
        // 在最后一个 "use crate::components::" 之后插入
        new_content = insert_after_last_match(
            &new_content,
            "use crate::components::",
            &format!("{}\n", import_line),
        );
    }

    // 2. 添加路由变体 (在 #[end_layout] 之前)
    let route_line = format!(
        "        #[route(\"/{}s\")]\n        {} {{}},",
        naming.resource, naming.component_name
    );
    if !new_content.contains(&route_line) {
        new_content = insert_before_line(
            &new_content,
            "    #[end_layout]",
            &format!("{}\n", route_line),
        );
    }

    std::fs::write(router_path, new_content)
        .map_err(|e| format!("写入 router/mod.rs 失败: {}", e))?;
    Ok(())
}

/// 注册前端 menu_item 路由映射
pub fn register_fe_menu_item(menu_item_path: &Path, naming: &Naming) -> Result<(), String> {
    let content = std::fs::read_to_string(menu_item_path)
        .map_err(|e| format!("读取 menu_item.rs 失败: {}", e))?;

    let route_line = format!(
        "        \"{}\" | \"{}s\" => Some(Route::{} {{}}),",
        naming.resource, naming.resource, naming.component_name
    );
    if content.contains(&route_line) {
        return Ok(());
    }

    // 在 "settings" => Some(Route::Settings {}), 之前插入
    let new_content = insert_before_line(
        &content,
        "\"settings\"",
        &format!("{}\n", route_line),
    );

    std::fs::write(menu_item_path, new_content)
        .map_err(|e| format!("写入 menu_item.rs 失败: {}", e))?;
    Ok(())
}

/// 注册前端 i18n
pub fn register_fe_i18n(
    i18n_path: &Path,
    config: &ModuleConfig,
    naming: &Naming,
) -> Result<(), String> {
    let content = std::fs::read_to_string(i18n_path)
        .map_err(|e| format!("读取 i18n/mod.rs 失败: {}", e))?;

    let mut new_content = content;

    // 1. 添加 TKey 变体 (在最后一个 TKey 变体之后，即在 "}" 之前)
    let keys = crate::frontend_gen::gen_i18n_keys(config, naming);
    let key_variants: Vec<String> = keys.iter().map(|k| k.clone()).collect();
    let _key_line = format!(",\n    {}", key_variants.join(", "));

    // 找到 TKey 枚举的结束位置 "}" — 需要找到 "// 个人信息和设置" 之后的最后一个 "}"
    if !new_content.contains(&keys[0]) {
        // 在 "ThemeColor, PrimaryColor, SuccessColor, WarningColor, DangerColor," 之后添加
        let anchor = "DangerColor,";
        new_content = insert_after_line(&new_content, anchor, &format!("    {}\n", key_variants.join(", ")));
    }

    // 2. 添加中文翻译
    let zh_pairs = crate::frontend_gen::gen_i18n_zh(config, naming);
    let zh_lines: Vec<String> = zh_pairs
        .iter()
        .map(|(k, v)| format!("        TKey::{} => \"{}\",", k, v))
        .collect();
    if !new_content.contains(&format!("TKey::{}", keys[0])) {
        // 在 t_zh 函数的最后一个翻译行之后插入
        // 找到 "TKey::DangerColor =>" 行
        let zh_anchor = "TKey::DangerColor =>";
        let zh_insertion = format!("{}\n", zh_lines.join("\n"));
        new_content = insert_after_line(&new_content, zh_anchor, &zh_insertion);
    }

    // 3. 添加英文翻译
    let en_pairs = crate::frontend_gen::gen_i18n_en(config, naming);
    let en_lines: Vec<String> = en_pairs
        .iter()
        .map(|(k, v)| format!("        TKey::{} => \"{}\",", k, v))
        .collect();
    // 检查是否已存在
    let en_check = format!("TKey::{} => \"{}\"", keys[0], en_pairs[0].1);
    if !new_content.contains(&en_check) {
        // 找到 t_en 函数中的 "TKey::DangerColor =>" 行
        // 注意: 文件中有两个 "TKey::DangerColor =>" (zh 和 en)，需要找第二个
        let en_anchor = "TKey::DangerColor =>";
        let en_insertion = format!("{}\n", en_lines.join("\n"));

        // 找到第二个出现的位置
        let first_pos = new_content.find(en_anchor);
        if let Some(first) = first_pos {
            let after_first = &new_content[first + en_anchor.len()..];
            if let Some(second) = after_first.find(en_anchor) {
                let abs_pos = first + en_anchor.len() + second;
                let line_end = new_content[abs_pos..].find('\n').map(|i| abs_pos + i + 1).unwrap_or(new_content.len());
                let mut result = new_content[..line_end].to_string();
                result.push_str(&en_insertion);
                result.push_str(&new_content[line_end..]);
                new_content = result;
            }
        }
    }

    std::fs::write(i18n_path, new_content)
        .map_err(|e| format!("写入 i18n/mod.rs 失败: {}", e))?;
    Ok(())
}
