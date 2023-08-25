use std::sync::Arc;

use common::contextx::AppContext;

use crate::ability::{new_tag_create_ability, new_tag_update_ability};

use self::{itag_service::ITagService, service::tag_service::TagService};

pub mod service;

pub mod itag_service;

pub fn new_tag_service(ctx: Arc<AppContext>) -> impl ITagService {
    TagService {
        ctx: ctx.clone(),
        tag_create_ability: new_tag_create_ability(ctx.clone()),
        tag_update_ability: new_tag_update_ability(ctx.clone()),
    }
}
