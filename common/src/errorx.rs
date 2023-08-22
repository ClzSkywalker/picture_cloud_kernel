use std::fmt::{self, Display};

use crate::i18n::{I18nKey, Locale};

#[derive(Debug, Clone, Default)]
pub struct Errorx {
    pub id: I18nKey,
    pub msg: String,
}

impl Display for Errorx {
    // IntellijRust does not understand that [non_exhaustive] applies only for downstream crates
    // noinspection RsMatchCheck
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Errorx {
    pub fn new(locale: Locale, id: I18nKey) -> Self {
        let mut e = Errorx::default();
        e.id = id;
        e.msg = e.id.trans(locale);
        e
    }
}

// #[derive(Debug, Clone, Copy, strum_macros::Display)]
// pub enum Locale {
//     En,
//     Zh,
// }

// impl Default for Locale {
//     fn default() -> Self {
//         Locale::En
//     }
// }

// impl From<&str> for Locale {
//     fn from(value: &str) -> Self {
//         if value == Locale::En.to_string().as_str() {
//             return Locale::En;
//         }
//         Locale::Zh
//     }
// }

// #[derive(Debug, Clone, Copy, strum_macros::Display)]
// pub enum I18nKey {
//     Success = 0,
//     DbCreate = 101001,
//     DbInit = 101002,
// }

// impl Default for I18nKey {
//     fn default() -> Self {
//         I18nKey::Success
//     }
// }
