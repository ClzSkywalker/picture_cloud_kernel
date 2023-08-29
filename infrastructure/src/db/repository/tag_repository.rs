use super::super::converter::preclude::*;
use super::super::model::preclude::*;
use base::ddd::repository::IRepository;
use chrono::Local;
use common::contextx::AppContext;
use common::errorx::Errorx;
use domain::aggregate::{preclude::*, tag::repository::itag_repository::ITagRespository};

use sea_orm::{prelude::*, ActiveValue::NotSet, DbBackend, QuerySelect, Set, Statement};
use sea_query::Condition;

const QUERY_PREV_SQL: &str = r#"WITH RECURSIVE cte AS (
    SELECT t1.*
    FROM [table] t1
    WHERE id = [value] and t1.deleted_at ISNULL
    UNION ALL
    SELECT t2.* 
    FROM [table] t2
    JOIN cte t3 ON t3.parent_id = t2.id WHERE t2.deleted_at ISNULL
)
  SELECT *
  FROM cte; "#;

const QUERY_NEXT_SQL: &str = r#"WITH RECURSIVE cte AS (
    SELECT t1.*
    FROM [table] t1
    WHERE id = [value] and t1.deleted_at ISNULL
    UNION ALL
    SELECT t2.* 
    FROM [table] t2
    JOIN cte t3 ON t2.parent_id = t3.id WHERE t2.deleted_at ISNULL
  )
  SELECT *
  FROM cte;"#;

pub struct TagRepository {}

impl TagRepository {
    async fn find_prevs(&self, ctx: &mut AppContext, id: i32) -> anyhow::Result<Vec<TagInfoModel>> {
        let sql = QUERY_PREV_SQL
            .replace("[table]", TagInfoModel::table_name().as_str())
            .replace("[value]", id.to_string().as_str());

        let active =
            TagInfoEntity::find().from_raw_sql(Statement::from_string(DbBackend::Sqlite, sql));

        let res = match &ctx.tx {
            Some(r) => active.all(r).await,
            None => active.all(&ctx.db).await,
        };

        match res {
            Ok(r) => Ok(r),
            Err(e) => {
                tracing::error!("{},e:{},data:{:?}", ctx.to_string(), e, id);
                anyhow::bail!(e);
            }
        }
    }

    async fn find_nexts(&self, ctx: &mut AppContext, id: i32) -> anyhow::Result<Vec<TagInfoModel>> {
        let sql = QUERY_NEXT_SQL
            .replace("[table]", TagInfoModel::table_name().as_str())
            .replace("[value]", id.to_string().as_str());

        let active =
            TagInfoEntity::find().from_raw_sql(Statement::from_string(DbBackend::Sqlite, sql));

        let res = match &ctx.tx {
            Some(r) => active.all(r).await,
            None => active.all(&ctx.db).await,
        };

        match res {
            Ok(r) => Ok(r),
            Err(e) => {
                tracing::error!("{},e:{},data:{:?}", ctx.to_string(), e, id);
                anyhow::bail!(e);
            }
        }
    }
}

#[async_trait::async_trait]
impl IRepository for TagRepository {
    type AG = TagAggregate;
    type ID = i32;
    async fn insert(&self, ctx: &mut AppContext, ag: Self::AG) -> anyhow::Result<Self::AG> {
        // 查询父类是否存在
        let mut prevs: Vec<TagInfoModel> = Vec::new();
        if ag.parent_id != 0 {
            prevs = match __self.find_prevs(ctx, ag.parent_id).await {
                Ok(r) => r,
                Err(e) => {
                    anyhow::bail!(e)
                }
            };
            if prevs.is_empty() {
                anyhow::bail!(Errorx::new(
                    ctx.locale,
                    common::i18n::I18nKey::TagParentNotExist
                ))
            }
        }

        let mut m = TagSerialize(ag.clone());
        m.created_at = Some(Local::now());

        let mut active: TagInfoActive = m.into();
        active.id = NotSet;
        let res = match &ctx.tx {
            Some(r) => active.insert(r).await,
            None => active.insert(&ctx.db).await,
        };

        let tag = match res {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("{},e:{},data:{:?}", ctx.to_string(), e, ag);
                anyhow::bail!(e);
            }
        };

        Ok(TagDeserialize(tag, Some(prevs), None))
    }

    async fn delete(&self, ctx: &mut AppContext, id: Self::ID) -> anyhow::Result<()> {
        let active = TagInfoEntity::update(TagInfoActive {
            id: Set(id.clone()),
            deleted_at: Set(Some(Local::now())),
            ..Default::default()
        })
        .filter(Condition::all().add(Expr::col(TagInfoColumn::DeletedAt).is_null()));
        let res = match &ctx.tx {
            Some(r) => active.exec(r).await,
            None => active.exec(&ctx.db).await,
        };

        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("{},e:{},data:{:?}", ctx.to_string(), e, id);
                anyhow::bail!(e);
            }
        }
    }

    async fn update(&self, ctx: &mut AppContext, ag: Self::AG) -> anyhow::Result<()> {
        let mut m = TagSerialize(ag.clone());
        m.updated_at = Some(Local::now());

        let active: TagInfoActive = m.into();
        let res = match &ctx.tx {
            Some(r) => active.update(r).await,
            None => active.update(&ctx.db).await,
        };
        match res {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("{},e:{},data:{:?}", ctx.to_string(), e, ag);
                anyhow::bail!(e);
            }
        };
        Ok(())
    }

    async fn by_id(&self, ctx: &mut AppContext, id: Self::ID) -> anyhow::Result<Option<Self::AG>> {
        let active = TagInfoEntity::find_by_id(id.clone())
            .filter(Condition::all().add(Expr::col(TagInfoColumn::DeletedAt).is_null()))
            .limit(1);
        let res = match &ctx.tx {
            Some(r) => active.one(r).await,
            None => active.one(&ctx.db).await,
        };
        let tag: TagInfoModel = match res {
            Ok(r) => match r {
                Some(r) => r,
                None => return Ok(None),
            },
            Err(e) => {
                tracing::error!("{:?},e:{},data:{:?}", ctx, e, id);
                anyhow::bail!(e)
            }
        };

        let prevs = match __self.find_prevs(ctx, id).await {
            Ok(r) => r,
            Err(e) => {
                anyhow::bail!(e)
            }
        };

        let nexts = match __self.find_nexts(ctx, id).await {
            Ok(r) => r,
            Err(e) => {
                anyhow::bail!(e)
            }
        };

        Ok(Some(TagDeserialize(tag, Some(prevs), Some(nexts))))
    }
}

#[async_trait::async_trait]
impl ITagRespository for TagRepository {
    async fn del_by_ids(&self, ctx: &mut AppContext, ids: Vec<i32>) -> anyhow::Result<()> {
        let model = TagInfoActive {
            deleted_at: Set(Some(Local::now())),
            ..Default::default()
        };
        let active = TagInfoEntity::update_many().set(model).filter(
            Condition::all()
                .add(Expr::col(TagInfoColumn::DeletedAt).is_null())
                .add(Expr::col(TagInfoColumn::Id).is_in(ids.clone())),
        );

        let res = match &ctx.tx {
            Some(r) => active.exec(r).await,
            None => active.exec(&ctx.db).await,
        };

        match res {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("{},e:{},data:{:?}", ctx, e, ids);
                anyhow::bail!(e)
            }
        };
        Ok(())
    }

    async fn update_parent_by_ids(
        &self,
        ctx: &mut AppContext,
        ids: Vec<i32>,
        parent_id: i32,
    ) -> anyhow::Result<()> {
        let model = TagInfoActive {
            parent_id: Set(parent_id),
            ..Default::default()
        };
        let active = TagInfoEntity::update_many().set(model).filter(
            Condition::all()
                .add(Expr::col(TagInfoColumn::DeletedAt).is_null())
                .add(Expr::col(TagInfoColumn::Id).is_in(ids.clone())),
        );

        let res = match &ctx.tx {
            Some(r) => active.exec(r).await,
            None => active.exec(&ctx.db).await,
        };

        match res {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("{},e:{},data:{:?}", ctx.to_string(), e, ids);
                anyhow::bail!(e)
            }
        };
        Ok(())
    }

    async fn find_by_name(
        &self,
        ctx: &mut AppContext,
        name: String,
    ) -> anyhow::Result<Option<TagAggregate>> {
        let active = TagInfoEntity::find()
            .filter(
                Condition::all()
                    .add(Expr::col(TagInfoColumn::Name).eq(name.clone()))
                    .add(Expr::col(TagInfoColumn::DeletedAt).is_null()),
            )
            .limit(1);

        let res = match &ctx.tx {
            Some(r) => active.one(r).await,
            None => active.one(&ctx.db).await,
        };

        let tag = match res {
            Ok(r) => match r {
                Some(r) => r,
                None => {
                    return Ok(None);
                }
            },
            Err(e) => {
                tracing::error!("{},e:{},data:{:?}", ctx.to_string(), e, name);
                anyhow::bail!(e)
            }
        };

        let prevs = match __self.find_prevs(ctx, tag.parent_id).await {
            Ok(r) => r,
            Err(e) => {
                anyhow::bail!(e)
            }
        };

        let nexts = match __self.find_nexts(ctx, tag.id).await {
            Ok(r) => r,
            Err(e) => {
                anyhow::bail!(e)
            }
        };

        Ok(Some(TagDeserialize(tag, Some(prevs), Some(nexts))))
    }

    async fn exist_name(&self, ctx: &mut AppContext, name: String) -> anyhow::Result<bool> {
        let active = TagInfoEntity::find().filter(
            Condition::all()
                .add(Expr::col(TagInfoColumn::Name).eq(name.clone()))
                .add(Expr::col(TagInfoColumn::DeletedAt).is_null()),
        );

        let res = match &ctx.tx {
            Some(r) => active.count(r).await,
            None => active.count(&ctx.db).await,
        };

        match res {
            Ok(r) => Ok(r > 0),
            Err(e) => {
                tracing::error!("{},e:{},data:{:?}", ctx.to_string(), e, name);
                anyhow::bail!(e)
            }
        }
    }

    async fn exist_parent_id(&self, ctx: &mut AppContext, id: i32) -> anyhow::Result<bool> {
        let active = TagInfoEntity::find_by_id(id)
            .filter(Condition::all().add(Expr::col(TagInfoColumn::DeletedAt).is_null()));

        let res = match &ctx.tx {
            Some(r) => active.count(r).await,
            None => active.count(&ctx.db).await,
        };

        match res {
            Ok(r) => Ok(r > 0),
            Err(e) => {
                tracing::error!("{},e:{},data:{:?}", ctx.to_string(), e, id);
                anyhow::bail!(e)
            }
        }
    }
}
