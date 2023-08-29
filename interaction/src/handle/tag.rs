use std::sync::Arc;

use application::{
    ability::tag::cmd::{tag_create_cmd::TagCreateCmd, tag_update_cmd::TagUpdateCmd},
    command::{itag_service::ITagService, new_tag_service},
    query::{model::tag::dto::TagInfoItem, new_tag_cqrs_service},
};
use axum::Extension;
use common::{
    contextx::AppContext,
    res::{err_to_resp, Responsex},
};
use middlewarex::validator::ValidatedJson;

pub async fn tag_create(
    Extension(mut ctx): Extension<AppContext>,
    ValidatedJson(cmd): ValidatedJson<TagCreateCmd>,
) -> Responsex<i32> {
    let mut server = new_tag_service();
    match server.create(&mut ctx, &cmd).await {
        Ok(r) => Responsex::ok_with_data(r),
        Err(e) => err_to_resp(e, ctx.locale),
    }
}

pub async fn tag_update(
    Extension(mut ctx): Extension<AppContext>,
    ValidatedJson(cmd): ValidatedJson<TagUpdateCmd>,
) -> Responsex<()> {
    let mut server = new_tag_service();
    match server.update(&mut ctx, &cmd).await {
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
