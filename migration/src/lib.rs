use std::{error::Error, fs, path::Path};

use rusqlite::Connection;
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_create_table;

pub struct Migrator;

impl Migrator {
    pub async fn create_db(db_path: &String) -> Result<(), Box<dyn Error>> {
        let path = Path::new(db_path);
        if path.exists() {
            return Ok(());
        }
        if let Some(parent) = path.parent() {
            match fs::create_dir_all(parent) {
                Ok(_) => {}
                Err(e) => return Err(Box::new(e)),
            };
        }

        match Connection::open_with_flags(
            path,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE | rusqlite::OpenFlags::SQLITE_OPEN_CREATE,
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration)]
    }
}
