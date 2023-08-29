use crate::ability::{new_tag_create_ability, new_tag_update_ability};

use self::{itag_service::ITagService, service::tag_service::TagService};

pub mod service;

pub mod itag_service;

pub fn new_tag_service() -> impl ITagService {
    TagService {
        tag_create_ability: new_tag_create_ability(),
        tag_update_ability: new_tag_update_ability(),
    }
}
