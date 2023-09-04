use sea_orm::{DatabaseConnection, DatabaseTransaction};
use std::{fmt::Display, sync::Arc};
use tokio::sync::Mutex;

use crate::{i18n::Locale, utils};

pub type SharedStateCtx = Arc<Mutex<AppContext>>;

pub fn new_ctx(ctx: AppContext) -> SharedStateCtx {
    Arc::new(Mutex::new(ctx))
}

#[derive(Debug)]
pub struct AppContext {
    pub db: DatabaseConnection,
    pub tx: Vec<DatabaseTransaction>,
    pub flow_id: String,
    pub uid: String,
    pub tid: String,
    pub locale: Locale,
}

impl AppContext {
    pub fn new(db: DatabaseConnection, locale: Locale) -> Self {
        Self {
            db,
            tx: Vec::new(),
            flow_id: utils::generate_ulid(),
            uid: "".to_string(),
            tid: "".to_string(),
            locale,
        }
    }

    pub fn get_tx(&self) -> Option<&DatabaseTransaction> {
        if self.tx.is_empty() {
            return None;
        }
        Some(&self.tx.last().unwrap())
    }
}

impl Into<SharedStateCtx> for AppContext {
    fn into(self) -> SharedStateCtx {
        SharedStateCtx::new(Mutex::new(self))
    }
}

impl Display for AppContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ctx:{{flow_id:{},uid:{},tid:{},locale:{})}}",
            self.flow_id, self.uid, self.tid, self.locale
        )
    }
}

impl Clone for AppContext {
    fn clone(&self) -> Self {
        #[cfg(not(feature = "mock"))]
        {
            Self {
                db: self.db.clone(),
                tx: Vec::new(),
                flow_id: self.flow_id.clone(),
                uid: self.uid.clone(),
                tid: self.tid.clone(),
                locale: self.locale,
            }
        }
        #[cfg(feature = "mock")]
        {
            Self {
                db: DatabaseConnection::Disconnected,
                tx: None,
                flow_id: self.flow_id.clone(),
                uid: self.uid.clone(),
                tid: self.tid.clone(),
                locale: self.locale.clone(),
            }
        }
    }
}
