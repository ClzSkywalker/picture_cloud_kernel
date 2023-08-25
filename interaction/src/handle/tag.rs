use std::sync::Arc;

use application::{
    ability::tag::cmd::TagCreateCmd,
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
    Extension(ctx): Extension<AppContext>,
    ValidatedJson(cmd): ValidatedJson<TagCreateCmd>,
) -> Responsex<i32> {
    let ctx = Arc::new(ctx);
    let mut server = new_tag_service(ctx.clone());
    match server.create(&cmd).await {
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
