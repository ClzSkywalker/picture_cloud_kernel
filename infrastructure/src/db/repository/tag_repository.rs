use std::sync::Arc;

use super::super::converter::preclude::*;
use super::super::model::preclude::*;
use base::ddd::repository::IRepository;
use chrono::Local;
use common::contextx::AppContext;
use domain::aggregate::{preclude::*, tag::repository::itag_repository::ITagRespository};

use sea_orm::{prelude::*, DbBackend, QuerySelect, Set, Statement};
use sea_query::Condition;

const QUERY_PREV_SQL: &str = r#"WITH RECURSIVE cte AS (
    SELECT *
    FROM $1
    WHERE id = $2
    UNION ALL
    SELECT *
    FROM $3
    JOIN cte ON cte.parent_id = a.id
)
  SELECT *
  FROM cte; "#;

const QUERY_NEXT_SQL: &str = r#"WITH RECURSIVE cte AS (
    SELECT *
    FROM $1
    WHERE id = $2
    UNION ALL
    SELECT *
    FROM $3
    JOIN cte ON $4.parent_id = cte.id
  )
  SELECT *
  FROM cte;"#;

pub struct TagRepository {
    pub ctx: Arc<AppContext>,
}

#[async_trait::async_trait]
impl IRepository for TagRepository {
    type AG = TagAggregate;
    type ID = i32;
    async fn insert(&self, ag: Self::AG) -> anyhow::Result<Self::AG> {
        let mut m = TagSerialize(ag.clone());
        m.created_at = Some(Local::now());

        let active: TagInfoActive = m.into();
        let res = match &self.ctx.tx {
            Some(r) => active.insert(r).await,
            None => active.insert(&self.ctx.db).await,
        };

        match res {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("{},e:{},model:{:?}", self.ctx.to_string(), e, ag);
                anyhow::bail!(e);
            }
        };

        let active = TagInfoEntity::find().from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            QUERY_PREV_SQL,
            [
                TagInfoModel::table_name().into(),
                ag.parent_id.into(),
                TagInfoModel::table_name().into(),
            ],
        ));

        let res = match &self.ctx.tx {
            Some(r) => active.all(r).await,
            None => active.all(&self.ctx.db).await,
        };

        let parent_ids: Vec<i32> = match res {
            Ok(r) => r.iter().map(|item| item.id).collect(),
            Err(e) => {
                tracing::error!("{},e:{},model:{:?}", self.ctx.to_string(), e, ag);
                anyhow::bail!(e);
            }
        };

        let mut ag = ag.clone();
        ag.prev = Some(parent_ids);

        Ok(ag)
    }

    async fn delete(&self, id: Self::ID) -> anyhow::Result<()> {
        let active = TagInfoEntity::update(TagInfoActive {
            id: Set(id.clone()),
            deleted_at: Set(Some(Local::now())),
            ..Default::default()
        })
        .filter(Condition::all().add(Expr::col(TagInfoColumn::DeletedAt).is_null()));
        let res = match &self.ctx.tx {
            Some(r) => active.exec(r).await,
            None => active.exec(&self.ctx.db).await,
        };

        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("{},e:{},model:{:?}", self.ctx.to_string(), e, id);
                anyhow::bail!(e);
            }
        }
    }

    async fn update(&self, ag: Self::AG) -> anyhow::Result<()> {
        let mut m = TagSerialize(ag.clone());
        m.updated_at = Some(Local::now());

        let active: TagInfoActive = m.into();
        let res = match &self.ctx.tx {
            Some(r) => active.update(r).await,
            None => active.update(&self.ctx.db).await,
        };
        match res {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("{},e:{},model:{:?}", self.ctx.to_string(), e, ag);
                anyhow::bail!(e);
            }
        };
        Ok(())
    }

    async fn by_id(&self, id: Self::ID) -> anyhow::Result<Option<Self::AG>> {
        let active = TagInfoEntity::find_by_id(id.clone())
            .filter(Condition::all().add(Expr::col(TagInfoColumn::DeletedAt).is_null()))
            .limit(1);
        let res = match &__self.ctx.tx {
            Some(r) => active.one(r).await,
            None => active.one(&self.ctx.db).await,
        };
        let tag: TagInfoModel = match res {
            Ok(r) => match r {
                Some(r) => r,
                None => return Ok(None),
            },
            Err(e) => {
                tracing::error!("{:?},e:{},model:{:?}", self.ctx, e, id);
                anyhow::bail!(e)
            }
        };

        let active = TagInfoEntity::find().from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            QUERY_PREV_SQL,
            [
                TagInfoModel::table_name().into(),
                tag.parent_id.into(),
                TagInfoModel::table_name().into(),
            ],
        ));

        let res = match &self.ctx.tx {
            Some(r) => active.all(r).await,
            None => active.all(&self.ctx.db).await,
        };

        let prevs = match res {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("{},e:{},model:{:?}", self.ctx.to_string(), e, id);
                anyhow::bail!(e);
            }
        };

        let active = TagInfoEntity::find().from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            QUERY_NEXT_SQL,
            [
                TagInfoModel::table_name().into(),
                tag.id.into(),
                TagInfoModel::table_name().into(),
                TagInfoModel::table_name().into(),
            ],
        ));

        let res = match &self.ctx.tx {
            Some(r) => active.all(r).await,
            None => active.all(&self.ctx.db).await,
        };

        let nexts = match res {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("{},e:{},model:{:?}", self.ctx.to_string(), e, id);
                anyhow::bail!(e);
            }
        };

        Ok(Some(TagDeserialize(tag, prevs, nexts)))
    }
}

#[async_trait::async_trait]
impl ITagRespository for TagRepository {}
