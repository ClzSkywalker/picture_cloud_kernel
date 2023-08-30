#[cfg(test)]
mod tests {
    use application::ability::{new_tag_create_ability, tag::cmd::tag_create_cmd::TagCreateCmd};
    use base::ddd::ability::IAbility;
    use common::i18n::I18nKey;

    use crate::{async_method, prepare};

    // 正常情况
    #[test]
    pub fn success() {
        async_method(async {
            let mut ctx = prepare().await;

            let mut ability = new_tag_create_ability();

            let cmd1 = TagCreateCmd {
                name: String::from("success1"),
                parent_id: 0,
                sort: 7,
            };
            match ability.execute_ability(&mut ctx, &cmd1).await {
                Ok(r) => {
                    assert!(r.id > 0, "插入失败");
                    assert_eq!(r.sort, 7);
                }
                Err(e) => {
                    assert_eq!(e.to_string(), "");
                }
            }
        })
    }

    // 名字重复
    #[test]
    fn create_name_repeat() {
        async_method(async {
            let mut ctx = prepare().await;

            let mut ability = new_tag_create_ability();

            let cmd1 = TagCreateCmd {
                name: String::from("t1"),
                parent_id: 0,
                sort: 7,
            };
            match ability.execute_ability(&mut ctx, &cmd1).await {
                Ok(r) => {
                    assert_eq!(r.id, 0);
                }
                Err(e) => {
                    assert_eq!(e.to_string(), I18nKey::TagNameExist.trans(ctx.locale));
                }
            }
        })
    }

    // 判断父标签是否存在
    #[test]
    fn parent_exist() {
        async_method(async {
            let mut ctx = prepare().await;

            let mut ability = new_tag_create_ability();

            // 成功
            let cmd1 = TagCreateCmd {
                name: String::from("success1"),
                parent_id: 1,
                sort: 7,
            };
            match ability.execute_ability(&mut ctx, &cmd1).await {
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
            match ability.execute_ability(&mut ctx, &cmd2).await {
                Ok(r) => {
                    assert!(r.id == 0);
                }
                Err(e) => {
                    assert_eq!(e.to_string(), I18nKey::TagParentNotExist.trans(ctx.locale));
                }
            };
        })
    }
}
