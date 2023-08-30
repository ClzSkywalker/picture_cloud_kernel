use std::{sync::OnceLock, time::Duration};

use migration::{Migrator, MigratorTrait};
use sea_orm::*;

pub static DB_LOCAL: OnceLock<DbConn> = OnceLock::new();

pub async fn init_db(db_path: &String) -> anyhow::Result<DbConn> {
    if !db_path.eq("sqlite::memory:") {
        match migration::Migrator::create_db(db_path).await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("{}", e.to_string());
                anyhow::bail!(e.to_string())
            }
        }
    }

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
            anyhow::bail!(e)
        }
    };

    tracing::info!("Database connected");

    match Migrator::up(&db, None).await {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("{}", e.to_string());
            anyhow::bail!(e)
        }
    };

    Ok(db)
}
