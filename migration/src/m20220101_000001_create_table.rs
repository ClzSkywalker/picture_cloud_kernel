use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FileInfo::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Base::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Base::CreatedAt).date_time())
                    .col(ColumnDef::new(Base::UpdatedAt).date_time())
                    .col(ColumnDef::new(Base::DeletedAt).date_time())
                    .col(ColumnDef::new(FileInfo::Name).string())
                    .col(ColumnDef::new(FileInfo::CreatedTime).date_time())
                    .col(ColumnDef::new(FileInfo::Classify).string())
                    .col(ColumnDef::new(FileInfo::FileType).string())
                    .col(ColumnDef::new(FileInfo::Width).integer())
                    .col(ColumnDef::new(FileInfo::Height).integer())
                    .col(ColumnDef::new(FileInfo::Score).integer())
                    .col(ColumnDef::new(FileInfo::Size).integer())
                    .col(ColumnDef::new(FileInfo::Color).json())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FileInfo::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Base {
    Id,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[derive(Iden)]
enum FileInfo {
    Table,
    Name,
    CreatedTime,
    Classify,
    FileType,
    Width,
    Height,
    Score,
    Size,
    Color,
}
