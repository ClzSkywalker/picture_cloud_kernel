#[cfg(test)]
mod tests {
    use application::ability::{
        new_tag_delete_ability,
        tag::cmd::tag_delete_cmd::{TagDeleteCmd, TAG_DEL_CHILD},
    };
    use base::ddd::{ability::IAbility, repository::IRepository};
    use infrastructure::db::repository::new_tag_repository;

    use crate::prepare;

    // 正常情况
    #[tokio::test]
    pub async fn del_child_no_child() {
        let ctx = prepare().await;

        let mut ability = new_tag_delete_ability(ctx.clone());

        // 处理不带子类
        let cmd = TagDeleteCmd {
            id: 6,
            del_type: TAG_DEL_CHILD,
        };

        match ability.execute_ability(&cmd).await {
            Ok(r) => {
                assert!(r.id > 0);
            }
            Err(e) => {
                assert_eq!(e.to_string(), "");
            }
        };

        let repository = new_tag_repository(ctx);
        match repository.by_id(6).await {
            Ok(r) => match r {
                Some(_) => {
                    panic!()
                }
                None => {}
            },
            Err(e) => {
                assert_eq!(e.to_string(), "");
            }
        };
    }

    #[tokio::test]
    pub async fn del_child_child() {
        let ctx = prepare().await;

        let mut ability = new_tag_delete_ability(ctx.clone());

        // 处理不带
        let cmd = TagDeleteCmd {
            id: 6,
            del_type: TAG_DEL_CHILD,
        };

        match ability.execute_ability(&cmd).await {
            Ok(r) => {
                assert!(r.id > 0);
            }
            Err(e) => {
                assert_eq!(e.to_string(), "");
            }
        };

        let repository = new_tag_repository(ctx);
        match repository.by_id(6).await {
            Ok(r) => match r {
                Some(_) => {
                    panic!()
                }
                None => {}
            },
            Err(e) => {
                assert_eq!(e.to_string(), "");
            }
        };
    }
}
