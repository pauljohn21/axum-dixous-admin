//! 前端代码生成模块
//!
//! 生成 3 个前端文件:
//! 1. Model 数据模型
//! 2. API 后端调用封装
//! 3. Component CRUD 页面组件

use crate::config::ModuleConfig;
use crate::naming::Naming;
use crate::type_map::{
    get_type_mapping, frontend_field_type, frontend_insert_type, frontend_update_type,
};

/// 生成前端 Model 文件
pub fn gen_model(config: &ModuleConfig, naming: &Naming) -> String {
    let struct_name = &naming.pascal_singular;
    let insert_name = format!("{}InsertDTO", naming.pascal_singular);
    let update_name = format!("{}UpdateDTO", naming.pascal_singular);

    // Model 字段
    let mut model_fields = String::new();
    model_fields.push_str("    pub id: i32,\n");
    for field in &config.fields {
        let fe_type = frontend_field_type(field);
        model_fields.push_str(&format!("    #[serde(default)]\n"));
        model_fields.push_str(&format!("    pub {}: {},\n", field.name, fe_type));
    }
    model_fields.push_str("    #[serde(default)]\n");
    model_fields.push_str("    pub created_at: Option<String>,\n");

    // InsertDTO 字段
    let mut insert_fields = String::new();
    for field in &config.fields {
        let fe_type = frontend_insert_type(field);
        let mapping = get_type_mapping(&field.field_type);
        if mapping.is_string {
            // 字符串类型: 必填
            insert_fields.push_str(&format!("    pub {}: {},\n", field.name, fe_type));
        } else {
            // 非字符串类型: 可选
            insert_fields.push_str(&format!("    #[serde(skip_serializing_if = \"Option::is_none\")]\n"));
            insert_fields.push_str(&format!("    pub {}: {},\n", field.name, fe_type));
        }
    }

    // UpdateDTO 字段 (全部可选)
    let mut update_fields = String::new();
    for field in &config.fields {
        let fe_type = frontend_update_type(field);
        update_fields.push_str(&format!("    #[serde(skip_serializing_if = \"Option::is_none\")]\n"));
        update_fields.push_str(&format!("    pub {}: {},\n", field.name, fe_type));
    }

    format!(
        r#"use serde::{{Deserialize, Serialize}};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct {struct_name} {{
{model_fields}}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {insert_name} {{
{insert_fields}}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {update_name} {{
{update_fields}}}
"#,
        struct_name = struct_name,
        model_fields = model_fields.trim_end_matches('\n'),
        insert_name = insert_name,
        insert_fields = insert_fields.trim_end_matches('\n'),
        update_name = update_name,
        update_fields = update_fields.trim_end_matches('\n'),
    )
}

/// 生成前端 API 文件
pub fn gen_api(_config: &ModuleConfig, naming: &Naming) -> String {
    let model_module = &naming.fe_file;
    let api_path = &naming.resource;
    let struct_name = &naming.pascal_singular;
    let insert_dto = format!("{}InsertDTO", naming.pascal_singular);
    let update_dto = format!("{}UpdateDTO", naming.pascal_singular);
    let delete_fn = format!("delete_{}", naming.resource);

    format!(
        r#"use crate::http::{{build_page_query, delete_void, get_with_query, post, put}};
use crate::models::common::PageResponse;
use crate::models::{model_module}::{{{struct_name}, {insert_dto}, {update_dto}}};

pub async fn list(
    page: Option<u32>,
    page_size: Option<u32>,
    keyword: Option<&str>,
) -> Result<PageResponse<{struct_name}>, String> {{
    let query = build_page_query(page, page_size, keyword);
    get_with_query("/api/{api_path}/list", &query).await
}}

pub async fn create(data: {insert_dto}) -> Result<(), String> {{
    post("/api/{api_path}", &data).await
}}

pub async fn update(id: i32, data: {update_dto}) -> Result<{struct_name}, String> {{
    put(&format!("/api/{api_path}/{{}}", id), &data).await
}}

pub async fn {delete_fn}(id: i32) -> Result<(), String> {{
    delete_void(&format!("/api/{api_path}/{{}}", id)).await
}}
"#,
        model_module = model_module,
        struct_name = struct_name,
        insert_dto = insert_dto,
        update_dto = update_dto,
        api_path = api_path,
        delete_fn = delete_fn,
    )
}

/// 生成前端组件文件
pub fn gen_component(config: &ModuleConfig, naming: &Naming) -> String {
    let component_name = &naming.component_name;
    let model_module = &naming.fe_file;
    let struct_name = &naming.pascal_singular;
    let insert_dto = format!("{}InsertDTO", naming.pascal_singular);
    let update_dto = format!("{}UpdateDTO", naming.pascal_singular);
    let api_module = &naming.fe_file;
    let delete_fn = format!("delete_{}", naming.resource);

    // TKey 变体 (使用 pascal_resource，无 Sys 前缀)
    let manage_key = format!("{}Manage", naming.pascal_resource);
    let add_key = format!("Add{}", naming.pascal_resource);
    let edit_key = format!("Edit{}", naming.pascal_resource);
    let search_key = format!("Search{}Placeholder", naming.pascal_resource);

    // 信号声明 (每个字段一个 form_ 信号)
    let mut signal_decls = String::new();
    for field in &config.fields {
        signal_decls.push_str(&format!(
            "    let mut form_{} = use_signal(String::new);\n",
            field.name
        ));
    }

    // fetch 函数中的 API 调用
    let fetch_call = format!(
        "            match api::{}::list(Some(current_page()), Some(page_size), Some(&kw)).await {{",
        api_module
    );

    // on_add 重置表单
    let mut on_add_resets = String::new();
    for field in &config.fields {
        on_add_resets.push_str(&format!(
            "        form_{}.set(String::new());\n",
            field.name
        ));
    }

    // on_edit 填充表单
    let mut on_edit_fills = String::new();
    for field in &config.fields {
        let mapping = get_type_mapping(&field.field_type);
        if mapping.is_string {
            on_edit_fills.push_str(&format!(
                "        form_{}.set(item.{}.clone().unwrap_or_default());\n",
                field.name, field.name
            ));
        } else if mapping.is_bool {
            on_edit_fills.push_str(&format!(
                "        form_{}.set(item.{}.map(|b| if b {{ \"true\".into() }} else {{ \"false\".into() }}).unwrap_or_default());\n",
                field.name, field.name
            ));
        } else {
            // 数值类型
            on_edit_fills.push_str(&format!(
                "        form_{}.set(item.{}.map(|v| v.to_string()).unwrap_or_default());\n",
                field.name, field.name
            ));
        }
    }

    // on_delete
    let on_delete = format!(
        r#"    let mut on_delete = move |id: i32| {{
        spawn(async move {{
            match api::{}::{}(id).await {{
                Ok(_) => {{ fetch_data(); }}
                Err(e) => {{ error_msg.set(Some(e)); }}
            }}
        }});
    }};"#,
        api_module, delete_fn
    );

    // on_submit 中的 InsertDTO 构造
    let mut insert_dto_fields = String::new();
    for field in &config.fields {
        let mapping = get_type_mapping(&field.field_type);
        let signal = format!("form_{}()", field.name);
        if mapping.is_string {
            insert_dto_fields.push_str(&format!(
                "                {}: {},\n",
                field.name, signal
            ));
        } else if mapping.is_bool {
            insert_dto_fields.push_str(&format!(
                "                {}: if {}.is_empty() {{ None }} else {{ Some({} == \"true\" || {} == \"1\") }},\n",
                field.name, signal, signal, signal
            ));
        } else if mapping.is_numeric {
            let parse_type = mapping.frontend_rust;
            insert_dto_fields.push_str(&format!(
                "                {}: {}.parse::<{}>().ok(),\n",
                field.name, signal, parse_type
            ));
        } else {
            insert_dto_fields.push_str(&format!(
                "                {}: if {}.is_empty() {{ None }} else {{ Some({}) }},\n",
                field.name, signal, signal
            ));
        }
    }

    // on_submit 中的 UpdateDTO 构造
    let mut update_dto_fields = String::new();
    for field in &config.fields {
        let mapping = get_type_mapping(&field.field_type);
        let signal = format!("form_{}()", field.name);
        if mapping.is_string {
            update_dto_fields.push_str(&format!(
                "                {}: if {}.is_empty() {{ None }} else {{ Some({}) }},\n",
                field.name, signal, signal
            ));
        } else if mapping.is_bool {
            update_dto_fields.push_str(&format!(
                "                {}: if {}.is_empty() {{ None }} else {{ Some({} == \"true\" || {} == \"1\") }},\n",
                field.name, signal, signal, signal
            ));
        } else if mapping.is_numeric {
            let parse_type = mapping.frontend_rust;
            update_dto_fields.push_str(&format!(
                "                {}: {}.parse::<{}>().ok(),\n",
                field.name, signal, parse_type
            ));
        } else {
            update_dto_fields.push_str(&format!(
                "                {}: if {}.is_empty() {{ None }} else {{ Some({}) }},\n",
                field.name, signal, signal
            ));
        }
    }

    // 表格列头
    let mut th_cells = String::new();
    th_cells.push_str(r#"                            th { style: "{th_s}", "ID" }"#);
    th_cells.push('\n');
    for field in &config.fields {
        let field_key = format!("{}{}", naming.pascal_resource, crate::naming::to_pascal(&field.name));
        th_cells.push_str(&format!(
            r#"                            th {{ style: "{{th_s}}", "{{t(TKey::{})}}" }}"#,
            field_key
        ));
        th_cells.push('\n');
    }
    th_cells.push_str(r#"                            th { style: "{th_s}", "{t(TKey::Action)}" }"#);

    // 表格行
    let mut td_cells = String::new();
    td_cells.push_str(r#"                                    td { style: "{td_s}", "{item.id}" }"#);
    td_cells.push('\n');
    for field in &config.fields {
        let mapping = get_type_mapping(&field.field_type);
        if mapping.is_string {
            td_cells.push_str(&format!(
                r#"                                    td {{ style: "{{td_s}}", "{{item.{}.clone().unwrap_or_default()}}" }}"#,
                field.name
            ));
        } else if mapping.is_bool {
            td_cells.push_str(&format!(
                r#"                                    td {{ style: "{{td_s}}", "{{item.{}.map(|b| if b {{ t(TKey::Enabled) }} else {{ t(TKey::Disabled) }}).unwrap_or_default()}}" }}"#,
                field.name
            ));
        } else {
            td_cells.push_str(&format!(
                r#"                                    td {{ style: "{{td_s}}", "{{item.{}.map(|v| v.to_string()).unwrap_or_default()}}" }}"#,
                field.name
            ));
        }
        td_cells.push('\n');
    }
    td_cells.push_str(r#"                                    td {
                                        style: "padding: 12px 16px;",
                                        div {
                                            style: "display: flex; gap: 8px;",
                                            Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Small), on_click: { let item = item.clone(); move |_| on_edit(item.clone()) }, "{t(TKey::Edit)}" }
                                            Button { variant: ButtonVariant::Danger, size: Some(ButtonSize::Small), on_click: move |_| on_delete(item.id), "{t(TKey::Delete)}" }
                                        }
                                    }"#);

    let colspan = config.fields.len() + 2; // ID + fields + Action

    // 对话框表单字段
    let mut form_inputs = String::new();
    for field in &config.fields {
        let field_key = format!("{}{}", naming.pascal_resource, crate::naming::to_pascal(&field.name));
        let placeholder_key = format!("{}{}Placeholder", naming.pascal_resource, crate::naming::to_pascal(&field.name));
        let signal = format!("form_{}", field.name);
        form_inputs.push_str(&format!(
            r#"                        div {{
                            style: "margin-bottom: 16px;",
                            label {{ style: "display: block; font-size: 14px; color: var(--el-text-color-regular); margin-bottom: 8px;", "{{t(TKey::{})}}" }}
                            Input {{
                                value: Some({signal}()),
                                placeholder: Some(t(TKey::{ph})),
                                on_change: move |e: Event<FormData>| {{ {signal}.set(e.data().value()); }}
                            }}
                        }}
"#,
            field_key,
            ph = placeholder_key,
            signal = signal,
        ));
    }

    format!(
        r#"use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::api;
use crate::i18n::{{t, t_paging, TKey}};
use crate::models::{model_module}::{{{struct_name}, {insert_dto}, {update_dto}}};

/// {module_cn} 页面
#[component]
pub fn {component_name}() -> Element {{
    let mut data_list = use_signal(Vec::new);
    let mut total = use_signal(|| 0u64);
    let mut current_page = use_signal(|| 1u32);
    let page_size = 10u32;
    let mut keyword = use_signal(String::new);
    let mut loading = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);

    let mut dialog_visible = use_signal(|| false);
    let mut is_edit = use_signal(|| false);
    let mut edit_id = use_signal(|| 0i32);
{signal_decls}
    let mut fetch_data = move || {{
        loading.set(true);
        error_msg.set(None);
        let kw = keyword();
        spawn(async move {{
{fetch_call}
                Ok(resp) => {{
                    data_list.set(resp.list);
                    total.set(resp.total);
                }}
                Err(e) => {{ error_msg.set(Some(e)); }}
            }}
            loading.set(false);
        }});
    }};

    use_effect(move || {{ fetch_data(); }});

    let on_add = move |_| {{
        is_edit.set(false);
{on_add_resets}        dialog_visible.set(true);
    }};

    let mut on_edit = move |item: {struct_name}| {{
        is_edit.set(true);
        edit_id.set(item.id);
{on_edit_fills}        dialog_visible.set(true);
    }};

{on_delete}

    let on_submit = move |_| {{
        if is_edit() {{
            let dto = {update_dto} {{
{update_dto_fields}            }};
            let id = edit_id();
            spawn(async move {{
                match api::{api_module}::update(id, dto).await {{
                    Ok(_) => {{ dialog_visible.set(false); fetch_data(); }}
                    Err(e) => {{ error_msg.set(Some(e)); }}
                }}
            }});
        }} else {{
            let dto = {insert_dto} {{
{insert_dto_fields}            }};
            spawn(async move {{
                match api::{api_module}::create(dto).await {{
                    Ok(_) => {{ dialog_visible.set(false); fetch_data(); }}
                    Err(e) => {{ error_msg.set(Some(e)); }}
                }}
            }});
        }}
    }};

    let total_pages = (total() + page_size as u64 - 1) / page_size as u64;

    let th_s = "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: var(--el-text-color-secondary); background: var(--el-fill-color-lighter); border-bottom: 1px solid var(--el-border-color-lighter);";
    let td_s = "padding: 12px 16px; font-size: 14px; color: var(--el-text-color-regular);";

    rsx! {{
        div {{
            div {{
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;",
                h2 {{ style: "font-size: 20px; font-weight: 600; color: var(--el-text-color-primary); margin: 0;", "{{t(TKey::{manage_key})}}" }}
                Button {{ variant: ButtonVariant::Primary, on_click: on_add, "{{t(TKey::{add_key})}}" }}
            }}

            if let Some(msg) = error_msg() {{
                div {{ style: "background: var(--el-color-danger-light-9); color: var(--el-color-danger); border-radius: 4px; padding: 10px 16px; margin-bottom: 16px; font-size: 14px;", "{{msg}}" }}
            }}

            div {{
                style: "display: flex; gap: 12px; margin-bottom: 20px; background: var(--el-bg-color); padding: 16px; border-radius: 8px; box-shadow: var(--el-box-shadow-light);",
                div {{
                    style: "flex: 1; max-width: 300px;",
                    Input {{
                        value: Some(keyword()),
                        placeholder: Some(t(TKey::{search_key})),
                        on_change: move |e: Event<FormData>| {{ keyword.set(e.data().value()); }}
                    }}
                }}
                Button {{ variant: ButtonVariant::Primary, on_click: move |_| {{ current_page.set(1); fetch_data(); }}, "{{t(TKey::Search)}}" }}
            }}

            div {{
                style: "background: var(--el-bg-color); border-radius: 8px; box-shadow: var(--el-box-shadow-light); overflow: hidden;",
                table {{
                    style: "width: 100%; border-collapse: collapse;",
                    thead {{
                        tr {{
{th_cells}
                        }}
                    }}
                    tbody {{
                        if loading() {{
                            tr {{ td {{ colspan: "{colspan}", style: "text-align: center; padding: 40px; color: var(--el-text-color-secondary);", "{{t(TKey::Loading)}}" }} }}
                        }} else if data_list().is_empty() {{
                            tr {{ td {{ colspan: "{colspan}", style: "text-align: center; padding: 40px; color: var(--el-text-color-secondary);", "{{t(TKey::NoData)}}" }} }}
                        }} else {{
                            for item in data_list() {{
                                tr {{
                                    style: "border-bottom: 1px solid var(--el-border-color-lighter);",
{td_cells}
                                }}
                            }}
                        }}
                    }}
                }}

                div {{
                    style: "display: flex; justify-content: space-between; align-items: center; padding: 16px 20px; border-top: 1px solid var(--el-border-color-lighter);",
                    span {{ style: "font-size: 14px; color: var(--el-text-color-secondary);", "{{t_paging(total(), current_page(), total_pages)}}" }}
                    div {{
                        style: "display: flex; gap: 8px;",
                        Button {{ variant: ButtonVariant::Default, size: Some(ButtonSize::Small), disabled: current_page() <= 1, on_click: move |_| {{ current_page.set(current_page() - 1); fetch_data(); }}, "{{t(TKey::PrevPage)}}" }}
                        Button {{ variant: ButtonVariant::Default, size: Some(ButtonSize::Small), disabled: current_page() >= total_pages as u32, on_click: move |_| {{ current_page.set(current_page() + 1); fetch_data(); }}, "{{t(TKey::NextPage)}}" }}
                    }}
                }}
            }}

            if dialog_visible() {{
                div {{
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: var(--el-overlay-color); z-index: 2000; display: flex; align-items: center; justify-content: center;",
                    onclick: move |_| {{ dialog_visible.set(false); }},
                    div {{
                        style: "background: var(--el-bg-color-overlay); border-radius: 8px; padding: 24px; width: 480px;",
                        onclick: move |e: MouseEvent| {{ e.stop_propagation(); }},
                        h3 {{ style: "font-size: 18px; font-weight: 600; color: var(--el-text-color-primary); margin: 0 0 24px 0;", if is_edit() {{ "{{t(TKey::{edit_key})}}" }} else {{ "{{t(TKey::{add_key})}}" }} }}
{form_inputs}                        div {{
                            style: "display: flex; justify-content: flex-end; gap: 12px;",
                            Button {{ variant: ButtonVariant::Default, on_click: move |_| {{ dialog_visible.set(false); }}, "{{t(TKey::Cancel)}}" }}
                            Button {{ variant: ButtonVariant::Primary, on_click: on_submit, "{{t(TKey::Confirm)}}" }}
                        }}
                    }}
                }}
            }}
        }}
    }}
}}
"#,
        module_cn = naming.module_cn,
        component_name = component_name,
        model_module = model_module,
        struct_name = struct_name,
        insert_dto = insert_dto,
        update_dto = update_dto,
        signal_decls = signal_decls,
        fetch_call = fetch_call,
        on_add_resets = on_add_resets,
        on_edit_fills = on_edit_fills,
        on_delete = on_delete,
        update_dto_fields = update_dto_fields,
        api_module = api_module,
        insert_dto_fields = insert_dto_fields,
        manage_key = manage_key,
        add_key = add_key,
        edit_key = edit_key,
        search_key = search_key,
        th_cells = th_cells,
        colspan = colspan,
        td_cells = td_cells,
        form_inputs = form_inputs,
    )
}

/// 生成 i18n TKey 变体列表
pub fn gen_i18n_keys(config: &ModuleConfig, naming: &Naming) -> Vec<String> {
    let mut keys = vec![
format!("{}Manage", naming.pascal_resource),
format!("Add{}", naming.pascal_resource),
format!("Edit{}", naming.pascal_resource),
format!("Search{}Placeholder", naming.pascal_resource),
    ];
    for field in &config.fields {
        let pascal = crate::naming::to_pascal(&field.name);
keys.push(format!("{}{}", naming.pascal_resource, pascal));
keys.push(format!("{}{}Placeholder", naming.pascal_resource, pascal));
    }
    keys
}

/// 生成中文翻译
pub fn gen_i18n_zh(config: &ModuleConfig, naming: &Naming) -> Vec<(String, String)> {
    let mut pairs = vec![
(format!("{}Manage", naming.pascal_resource), naming.module_cn.clone()),
(format!("Add{}", naming.pascal_resource), format!("+ 新增{}", naming.resource)),
(format!("Edit{}", naming.pascal_resource), format!("编辑{}", naming.resource)),
(format!("Search{}Placeholder", naming.pascal_resource), format!("搜索{}", naming.module_cn.trim_end_matches("管理"))),
    ];
    for field in &config.fields {
        let pascal = crate::naming::to_pascal(&field.name);
        let label = if field.comment.is_empty() {
            field.name.clone()
        } else {
            field.comment.clone()
        };
pairs.push((format!("{}{}", naming.pascal_resource, pascal), label.clone()));
pairs.push((format!("{}{}Placeholder", naming.pascal_resource, pascal), format!("请输入{}", label)));
    }
    pairs
}

/// 生成英文翻译
pub fn gen_i18n_en(config: &ModuleConfig, naming: &Naming) -> Vec<(String, String)> {
    let pascal_en = &naming.pascal_resource;
    let module_en = format!("{} Management", pascal_en);
    let mut pairs = vec![
        (format!("{}Manage", pascal_en), module_en.clone()),
        (format!("Add{}", pascal_en), format!("+ Add {}", pascal_en)),
        (format!("Edit{}", pascal_en), format!("Edit {}", pascal_en)),
        (format!("Search{}Placeholder", pascal_en), format!("Search {}", pascal_en)),
    ];
    for field in &config.fields {
        let pascal = crate::naming::to_pascal(&field.name);
        let label = if field.comment.is_empty() {
            crate::naming::to_pascal(&field.name)
        } else {
            // 将中文注释转为英文标签 (简单处理)
            crate::naming::to_pascal(&field.name)
        };
        pairs.push((format!("{}{}", pascal_en, pascal), label.clone()));
        pairs.push((format!("{}{}Placeholder", pascal_en, pascal), format!("Enter {}", label.to_lowercase())));
    }
    pairs
}
