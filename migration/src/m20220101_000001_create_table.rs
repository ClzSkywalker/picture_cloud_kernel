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
                    .col(
                        ColumnDef::new(Base::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
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
                    .col(ColumnDef::new(FileInfo::ClassifyId).json())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Classify::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Base::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Base::CreatedAt).date_time())
                    .col(ColumnDef::new(Base::UpdatedAt).date_time())
                    .col(ColumnDef::new(Base::DeletedAt).date_time())
                    .col(ColumnDef::new(Classify::Name).string())
                    .col(ColumnDef::new(Classify::ParentId).integer())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TagInfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Base::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Base::CreatedAt).date_time())
                    .col(ColumnDef::new(Base::UpdatedAt).date_time())
                    .col(ColumnDef::new(Base::DeletedAt).date_time())
                    .col(ColumnDef::new(TagInfo::Name).string())
                    .col(ColumnDef::new(TagInfo::ParentId).integer())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TagToFile::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Base::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Base::CreatedAt).date_time())
                    .col(ColumnDef::new(Base::UpdatedAt).date_time())
                    .col(ColumnDef::new(Base::DeletedAt).date_time())
                    .col(ColumnDef::new(TagToFile::Tid).integer())
                    .col(ColumnDef::new(TagToFile::Fid).integer())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Classify::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Base::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Base::CreatedAt).date_time())
                    .col(ColumnDef::new(Base::UpdatedAt).date_time())
                    .col(ColumnDef::new(Base::DeletedAt).date_time())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FileInfo::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Classify::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(TagInfo::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(TagToFile::Table).to_owned())
            .await?;
        Ok(())
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
    ClassifyId,
}

#[derive(Iden)]
enum Classify {
    Table,
    Name,
    ParentId,
}

#[derive(Iden)]
enum TagInfo {
    Table,
    Name,
    ParentId,
}

#[derive(Iden)]
enum TagToFile {
    Table,
    Tid,
    Fid,
}
