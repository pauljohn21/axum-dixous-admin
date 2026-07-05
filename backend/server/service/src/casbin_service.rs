use anyhow::{anyhow, Result};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect, ActiveModelTrait, Set, TransactionTrait};

use model::dao::casbin_rule;
use model::dto::page_dto::{PageRequest, PageResponse};
use model::prelude::CasbinRule;
use utils::db_conn;

pub struct CasbinService;

impl CasbinService {
    pub async fn list(query: PageRequest) -> Result<PageResponse<casbin_rule::Model>> {
        let db = db_conn!();
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(10);

        let mut q = CasbinRule::find();
        if let Some(keyword) = &query.keyword {
            q = q.filter(
                sea_orm::Condition::any()
                    .add(casbin_rule::Column::Ptype.contains(keyword))
                    .add(casbin_rule::Column::V0.contains(keyword))
                    .add(casbin_rule::Column::V1.contains(keyword))
                    .add(casbin_rule::Column::V2.contains(keyword))
                    .add(casbin_rule::Column::V3.contains(keyword))
                    .add(casbin_rule::Column::V4.contains(keyword))
                    .add(casbin_rule::Column::V5.contains(keyword)),
            );
        }

        let total = q.clone().count(db).await?;
        let list = q.offset((page - 1) * page_size).limit(page_size).all(db).await?;
        Ok(PageResponse { list, total, page, page_size })
    }

    pub async fn get_by_id(id: u64) -> Result<casbin_rule::Model> {
        CasbinRule::find_by_id(id)
            .one(db_conn!())
            .await?
            .ok_or_else(|| anyhow!("Casbin规则不存在"))
    }

    pub async fn create(rule: CreateCasbinRuleRequest) -> Result<casbin_rule::Model> {
        let db = db_conn!();
        
        let active_model = casbin_rule::ActiveModel {
            id: Set(0), // Auto increment
            ptype: Set(rule.ptype),
            v0: Set(rule.v0),
            v1: Set(rule.v1),
            v2: Set(rule.v2),
            v3: Set(rule.v3),
            v4: Set(rule.v4),
            v5: Set(rule.v5),
        };
        
        active_model.insert(db).await.map_err(|e| anyhow!("创建规则失败: {}", e))
    }

    pub async fn update(id: u64, rule: UpdateCasbinRuleRequest) -> Result<casbin_rule::Model> {
        let db = db_conn!();
        
        let existing_rule = CasbinRule::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| anyhow!("规则不存在"))?;
            
        let mut active_model: casbin_rule::ActiveModel = existing_rule.into();
        
        if let Some(ptype) = rule.ptype {
            active_model.ptype = Set(Some(ptype));
        }
        if let Some(v0) = rule.v0 {
            active_model.v0 = Set(Some(v0));
        }
        if let Some(v1) = rule.v1 {
            active_model.v1 = Set(Some(v1));
        }
        if let Some(v2) = rule.v2 {
            active_model.v2 = Set(Some(v2));
        }
        if let Some(v3) = rule.v3 {
            active_model.v3 = Set(Some(v3));
        }
        if let Some(v4) = rule.v4 {
            active_model.v4 = Set(Some(v4));
        }
        if let Some(v5) = rule.v5 {
            active_model.v5 = Set(Some(v5));
        }
        
        active_model.update(db).await.map_err(|e| anyhow!("更新规则失败: {}", e))
    }

    pub async fn delete(id: u64) -> Result<()> {
        let db = db_conn!();
        
        let result = CasbinRule::delete_by_id(id)
            .exec(db)
            .await?;
            
        if result.rows_affected == 0 {
            return Err(anyhow!("规则不存在或已被删除"));
        }
        
        Ok(())
    }

    pub async fn delete_batch(ids: Vec<u64>) -> Result<u64> {
        let db = db_conn!();
        
        let result = CasbinRule::delete_many()
            .filter(casbin_rule::Column::Id.is_in(ids))
            .exec(db)
            .await?;
            
        Ok(result.rows_affected)
    }

    /// 获取角色的权限策略
    pub async fn get_policy_by_role(role: &str) -> Result<Vec<casbin_rule::Model>> {
        let db = db_conn!();
        
        CasbinRule::find()
            .filter(casbin_rule::Column::Ptype.eq("p"))
            .filter(casbin_rule::Column::V0.eq(role))
            .all(db)
            .await
            .map_err(|e| anyhow!("查询角色策略失败: {}", e))
    }

    /// 获取用户的角色
    pub async fn get_roles_for_user(user: &str) -> Result<Vec<casbin_rule::Model>> {
        let db = db_conn!();
        
        CasbinRule::find()
            .filter(casbin_rule::Column::Ptype.eq("g"))
            .filter(casbin_rule::Column::V0.eq(user))
            .all(db)
            .await
            .map_err(|e| anyhow!("查询用户角色失败: {}", e))
    }

    /// 更新角色的权限策略
    pub async fn update_role_policies(role: &str, policies: Vec<(String, String)>) -> Result<()> {
        let db = db_conn!();
        
        let txn = db.begin().await?;
        
        // 删除现有策略
        CasbinRule::delete_many()
            .filter(casbin_rule::Column::Ptype.eq("p"))
            .filter(casbin_rule::Column::V0.eq(role))
            .exec(&txn)
            .await?;
            
        // 添加新策略
        for (obj, act) in policies {
            let active_model = casbin_rule::ActiveModel {
                id: Set(0),
                ptype: Set(Some("p".to_string())),
                v0: Set(Some(role.to_string())),
                v1: Set(Some(obj)),
                v2: Set(Some(act)),
                v3: Set(None),
                v4: Set(None),
                v5: Set(None),
            };
            active_model.insert(&txn).await?;
        }
        
        txn.commit().await?;
        Ok(())
    }

    /// 批量删除策略
    pub async fn delete_policies(&self, policies: Vec<Vec<String>>) -> Result<bool> {
        let db = db_conn!();
        
        for policy in policies {
            if policy.len() >= 3 {
                CasbinRule::delete_many()
                    .filter(casbin_rule::Column::Ptype.eq("p"))
                    .filter(casbin_rule::Column::V0.eq(&policy[0]))
                    .filter(casbin_rule::Column::V1.eq(&policy[1]))
                    .filter(casbin_rule::Column::V2.eq(&policy[2]))
                    .exec(db)
                    .await?;
            }
        }
        
        Ok(true)
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CreateCasbinRuleRequest {
    pub ptype: Option<String>,
    pub v0: Option<String>,
    pub v1: Option<String>,
    pub v2: Option<String>,
    pub v3: Option<String>,
    pub v4: Option<String>,
    pub v5: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct UpdateCasbinRuleRequest {
    pub ptype: Option<String>,
    pub v0: Option<String>,
    pub v1: Option<String>,
    pub v2: Option<String>,
    pub v3: Option<String>,
    pub v4: Option<String>,
    pub v5: Option<String>,
}
