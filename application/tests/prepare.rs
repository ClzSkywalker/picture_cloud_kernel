use chrono::Local;
use common::contextx::{AppContext, SharedStateCtx};
use infrastructure::db::model::preclude::*;
use sea_orm::{ActiveValue::NotSet, DbConn, EntityTrait};
use std::future::Future;
use tokio::sync::Mutex;

pub mod ability;

pub fn async_method<F: Future<Output = ()>>(callback: F) {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { callback.await })
}
pub async fn prepare() -> SharedStateCtx {
    let db = match infrastructure::db::model::common::init_db(&"sqlite::memory:".to_string()).await
    {
        Ok(r) => r,
        Err(e) => {
            panic!("{}", e.to_string());
        }
    };

    init_data(&db).await.unwrap();

    SharedStateCtx::new(Mutex::new(AppContext::new(db, common::i18n::Locale::En)))
}

async fn init_data(db: &DbConn) -> anyhow::Result<()> {
    let mut tag_entity_list: Vec<TagInfoModel> = Vec::with_capacity(6);
    for item in 1..=6 {
        let mut entity = TagInfoModel {
            id: item,
            created_at: Some(Local::now()),
            updated_at: None,
            deleted_at: None,
            name: format!("t{}", item),
            sort: item,
            parent_id: 0,
        };

        match item {
            3 => entity.parent_id = 1,
            4 => entity.parent_id = 3,
            5 => entity.parent_id = 2,
            6 => entity.deleted_at = Some(Local::now()),
            _ => {}
        };

        tag_entity_list.push(entity);
    }

    let tag_avtive_list: Vec<TagInfoActive> = tag_entity_list
        .into_iter()
        .map(|item| {
            let mut active: TagInfoActive = item.into();
            active.id = NotSet;
            active
        })
        .collect();

    match TagInfoEntity::insert_many(tag_avtive_list).exec(db).await {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e)
        }
    };
    Ok(())
}
