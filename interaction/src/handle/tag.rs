use std::sync::Arc;

use application::{
    ability::tag::cmd::{tag_create_cmd::TagCreateCmd, tag_update_cmd::TagUpdateCmd},
    command::{itag_service::ITagService, new_tag_service},
    query::{model::tag::dto::TagInfoItem, new_tag_cqrs_service},
};
use axum::Extension;
use common::{
    contextx::{AppContext, new_ctx},
    res::{err_to_resp, Responsex},
};
use middlewarex::validator::ValidatedJson;

pub async fn tag_create(
    Extension(ctx): Extension<AppContext>,
    ValidatedJson(cmd): ValidatedJson<TagCreateCmd>,
) -> Responsex<i32> {
    let ctx =new_ctx(ctx);
    let a=ctx.tx.lock().as_mut().unwrap();
    let a=ctx.tx.lock().unwrap().unwrap();
    let mut server = new_tag_service(ctx.clone());
    match server.create(&cmd).await {
        Ok(r) => Responsex::ok_with_data(r),
        Err(e) => err_to_resp(e, ctx.locale.clone()),
    }
}

pub async fn tag_update(
    Extension(ctx): Extension<AppContext>,
    ValidatedJson(cmd): ValidatedJson<TagUpdateCmd>,
) -> Responsex<()> {
    let ctx = Arc::new(ctx);
    let mut server = new_tag_service(ctx.clone());
    match server.update(&cmd).await {
        Ok(r) => Responsex::ok_with_data(r),
        Err(e) => err_to_resp(e, ctx.locale.clone()),
    }
}

pub async fn tag_find(Extension(ctx): Extension<AppContext>) -> Responsex<Vec<TagInfoItem>> {
    let ctx = Arc::new(ctx);
    let mut s = new_tag_cqrs_service(ctx.clone());
    match s.find().await {
        Ok(r) => Responsex::ok_with_data(r),
        Err(e) => err_to_resp(e, ctx.locale.clone()),
    }
}
