use domain::aggregate::preclude::*;
use domain::aggregate::tag::repository::itag_repository::ITagRespository;

use self::tag_repository::TagRepository;

pub mod tag_repository;

pub fn new_tag_repository() -> impl ITagRespository<AG = TagAggregate, ID = i32> {
    TagRepository {}
}
