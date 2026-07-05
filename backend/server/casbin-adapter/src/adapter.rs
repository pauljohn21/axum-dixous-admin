use async_trait::async_trait;
use casbin::{Adapter, Filter, Model, Result};
use sea_orm::ConnectionTrait;

use crate::{
    action::{self, Rule, RuleWithType},
    entity,
};

pub struct SeaOrmAdapter<C> {
    conn: C,
    is_filtered: bool,
}

impl<C: ConnectionTrait> SeaOrmAdapter<C> {
    pub async fn new(conn: C) -> Result<Self> {
        Ok(Self {
            conn,
            is_filtered: false,
        })
    }
}

impl<C> SeaOrmAdapter<C> {
    fn transform_policy_line<'a>(ptype: &'a str, rule: &'a [String]) -> Option<RuleWithType<'a>> {
        if ptype.trim().is_empty() || rule.is_empty() {
            return None;
        }

        Some(RuleWithType::from_rule(ptype, Rule::from_string(rule)))
    }

    fn normalize_policy(model: &entity::Model) -> Option<Vec<String>> {
        let mut policy = vec![
            &model.v0, &model.v1, &model.v2, &model.v3, &model.v4, &model.v5,
        ];

        loop {
            match policy.last() {
                Some(last) if last.is_empty() => {
                    policy.pop();
                }
                _ => break,
            }
        }

        if policy.is_empty() {
            None
        } else {
            Some(policy.iter().map(|&x| x.to_owned()).collect())
        }
    }
}

#[async_trait]
impl<C: ConnectionTrait + Send + Sync> Adapter for SeaOrmAdapter<C> {
    async fn load_policy(&mut self, m: &mut dyn Model) -> Result<()> {
        let rules = action::load_policy(&self.conn).await?;

        for rule in &rules {
            let Some(sec) = rule.ptype.chars().next().map(|x| x.to_string()) else {
                continue;
            };
            let Some(t1) = m.get_mut_model().get_mut(&sec) else {
                continue;
            };
            let Some(t2) = t1.get_mut(&rule.ptype) else {
                continue;
            };
            let Some(policy) = Self::normalize_policy(rule) else {
                continue;
            };
            t2.get_mut_policy().insert(policy);
        }

        Ok(())
    }

    async fn load_filtered_policy<'a>(&mut self, m: &mut dyn Model, f: Filter<'a>) -> Result<()> {
        let rules = action::load_filtered_policy(&self.conn, f).await?;
        self.is_filtered = true;

        for rule in &rules {
            let Some(sec) = rule.ptype.chars().next().map(|x| x.to_string()) else {
                continue;
            };
            let Some(t1) = m.get_mut_model().get_mut(&sec) else {
                continue;
            };
            let Some(t2) = t1.get_mut(&rule.ptype) else {
                continue;
            };
            let Some(policy) = Self::normalize_policy(rule) else {
                continue;
            };
            t2.get_mut_policy().insert(policy);
        }

        Ok(())
    }

    async fn save_policy(&mut self, m: &mut dyn Model) -> Result<()> {
        let mut rules = Vec::new();

        if let Some(map) = m.get_model().get("p") {
            for (ptype, assertion) in map {
                let new_rules = assertion
                    .get_policy()
                    .into_iter()
                    .filter_map(|x| Self::transform_policy_line(ptype, x));

                rules.extend(new_rules);
            }
        }

        if let Some(map) = m.get_model().get("g") {
            for (ptype, assertion) in map {
                let new_rules = assertion
                    .get_policy()
                    .into_iter()
                    .filter_map(|x| Self::transform_policy_line(ptype, x));

                rules.extend(new_rules);
            }
        }

        action::save_policies(&self.conn, rules).await
    }

    async fn clear_policy(&mut self) -> Result<()> {
        action::clear_policy(&self.conn).await
    }

    fn is_filtered(&self) -> bool {
        self.is_filtered
    }

    async fn add_policy(&mut self, _sec: &str, ptype: &str, rule: Vec<String>) -> Result<bool> {
        let Some(rule_with_type) = Self::transform_policy_line(ptype, rule.as_slice()) else {
            return Ok(false);
        };

        action::add_policy(&self.conn, rule_with_type).await
    }

    async fn add_policies(
        &mut self,
        _sec: &str,
        ptype: &str,
        rules: Vec<Vec<String>>,
    ) -> Result<bool> {
        let rules = rules
            .iter()
            .filter_map(|x| Self::transform_policy_line(ptype, x))
            .collect::<Vec<_>>();

        if rules.is_empty() {
            return Ok(false);
        }

        action::add_policies(&self.conn, rules).await
    }

    async fn remove_policy(&mut self, _sec: &str, ptype: &str, rule: Vec<String>) -> Result<bool> {
        let Some(rule_with_type) = Self::transform_policy_line(ptype, rule.as_slice()) else {
            return Ok(false);
        };

        action::remove_policy(&self.conn, rule_with_type).await
    }

    async fn remove_policies(
        &mut self,
        _sec: &str,
        ptype: &str,
        rules: Vec<Vec<String>>,
    ) -> Result<bool> {
        let rules = rules
            .iter()
            .filter_map(|x| Self::transform_policy_line(ptype, x))
            .collect::<Vec<_>>();

        if rules.is_empty() {
            return Ok(false);
        }

        action::remove_policies(&self.conn, rules).await
    }

    async fn remove_filtered_policy(
        &mut self,
        _sec: &str,
        ptype: &str,
        field_index: usize,
        field_values: Vec<String>,
    ) -> Result<bool> {
        if field_index <= 5 && !field_values.is_empty() && field_values.len() + field_index <= 6 {
            let rule = Rule::from_string(&field_values);
            action::remove_filtered_policy(&self.conn, ptype, field_index, rule).await
        } else {
            Ok(false)
        }
    }
}
