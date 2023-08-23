use std::{sync::OnceLock, time::Duration};

use common::{errorx::Errorx, i18n::I18nKey};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DbConn};

pub static DB_LOCAL: OnceLock<DbConn> = OnceLock::new();

pub async fn init_db(
    locale: common::i18n::Locale,
    db_path: &String,
) -> Result<DbConn, common::errorx::Errorx> {
    match migration::Migrator::create_db(db_path).await {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("{}", e.to_string());
            return Err(Errorx::new(locale, I18nKey::DbCreate));
        }
    }

    let db_path = "sqlite://".to_string() + db_path;
    let mut opt = ConnectOptions::new(db_path);
    opt.max_connections(1000)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .sqlx_logging_level(tracing::log::LevelFilter::Info)
        .sqlx_logging(true);

    let db: sea_orm::DatabaseConnection = match Database::connect(opt).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("{}", e.to_string());
            return Err(Errorx::new(locale, I18nKey::DbInit));
        }
    };

    tracing::info!("Database connected");

    match Migrator::up(&db, None).await {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("{}", e.to_string());
            return Err(Errorx::new(locale, I18nKey::DbUp));
        }
    };

    Ok(db)
}
