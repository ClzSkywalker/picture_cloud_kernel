#[cfg(test)]
mod tests {
    use application::ability::{
        new_tag_create_ability, new_tag_delete_ability,
        tag::cmd::{
            tag_create_cmd::TagCreateCmd,
            tag_delete_cmd::{TagDeleteCmd, TAG_DEL_CHILD},
        },
    };
    use base::ddd::ability::IAbility;
    use common::i18n::I18nKey;

    use crate::prepare;

    // success
    // 无父标签
    #[tokio::test]
    pub async fn no_parent_id() {
        let ctx = prepare().await;

        let mut ability = new_tag_create_ability(ctx);

        let cmd1 = TagCreateCmd {
            name: String::from("success1"),
            parent_id: 0,
            sort: 7,
        };
        match ability.execute_ability(&cmd1).await {
            Ok(r) => {
                assert!(r.is_not_empty(), "插入失败");
            }
            Err(e) => {
                assert_eq!(e.to_string(), "");
            }
        }
    }

    // 父标签id存在
    #[tokio::test]
    pub async fn parent_id_exist() {
        let ctx = prepare().await;

        let mut ability = new_tag_create_ability(ctx);

        let cmd1 = TagCreateCmd {
            name: String::from("success1"),
            parent_id: 1,
            sort: 7,
        };
        match ability.execute_ability(&cmd1).await {
            Ok(r) => {
                assert!(r.is_not_empty(), "插入失败");
            }
            Err(e) => {
                assert_eq!(e.to_string(), "");
            }
        }
    }

    // 跟已删除的tag名字重复
    #[tokio::test]
    pub async fn del_name_repeat() {
        let ctx = prepare().await;

        let mut ability = new_tag_create_ability(ctx.clone());

        let cmd1 = TagCreateCmd {
            name: String::from("success1"),
            parent_id: 1,
            sort: 7,
        };
        // 插入一条
        let tag1 = match ability.execute_ability(&cmd1).await {
            Ok(r) => {
                assert!(r.is_not_empty(), "插入失败");
                r
            }
            Err(e) => {
                assert_eq!(e.to_string(), "");
                panic!()
            }
        };

        // 删除上条
        let mut del_ability = new_tag_delete_ability(ctx.clone());
        match del_ability
            .execute_ability(&TagDeleteCmd {
                id: tag1.id,
                del_type: TAG_DEL_CHILD,
            })
            .await
        {
            Ok(r) => {
                assert!(r.is_not_empty(), "插入失败");
            }
            Err(e) => {
                assert_eq!(e.to_string(), "");
            }
        };

        // 插入老数据
        match ability.execute_ability(&cmd1).await {
            Ok(r) => {
                assert!(r.is_not_empty(), "插入失败");
            }
            Err(e) => {
                assert_eq!(e.to_string(), "");
            }
        };
    }

    // 名字重复
    #[tokio::test]
    pub async fn create_name_repeat() {
        let ctx = prepare().await;

        let mut ability = new_tag_create_ability(ctx.clone());

        let cmd1 = TagCreateCmd {
            name: String::from("t1"),
            parent_id: 0,
            sort: 7,
        };
        match ability.execute_ability(&cmd1).await {
            Ok(r) => {
                assert_eq!(r.id, 0);
            }
            Err(e) => {
                assert_eq!(
                    e.to_string(),
                    I18nKey::TagNameExist.trans(ctx.read().await.locale)
                );
            }
        }
    }

    // failure
    // 父标签id不存在
    #[tokio::test]
    pub async fn parent_exist() {
        let ctx = prepare().await;

        let mut ability = new_tag_create_ability(ctx.clone());

        // 成功
        let cmd1 = TagCreateCmd {
            name: String::from("success1"),
            parent_id: 1,
            sort: 7,
        };
        match ability.execute_ability(&cmd1).await {
            Ok(r) => {
                assert!(r.id > 0);
            }
            Err(e) => {
                assert_eq!(e.to_string(), "");
            }
        };

        // 失败
        let cmd2 = TagCreateCmd {
            name: String::from("failure"),
            parent_id: 999,
            sort: 7,
        };
        match ability.execute_ability(&cmd2).await {
            Ok(r) => {
                assert!(r.id == 0);
            }
            Err(e) => {
                assert_eq!(
                    e.to_string(),
                    I18nKey::TagParentNotExist.trans(ctx.read().await.locale)
                );
            }
        };
    }
}
