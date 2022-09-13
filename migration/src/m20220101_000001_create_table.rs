use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                        .integer()
                        .auto_increment()
                        .not_null()
                    )
                    .col(
                        ColumnDef::new(User::UserId)
                        .uuid()
                        .primary_key()
                    )
                    .col(
                        ColumnDef::new(User::Email)
                        .text()
                        .not_null()
                    )
                    .col(
                        ColumnDef::new(User::Password)
                        .text()
                        .not_null()
                    )
                    .col(
                        ColumnDef::new(User::Username)
                        .text()
                        .not_null()
                    )
                    .col(
                        ColumnDef::new(User::Avatar)
                        .text()
                        .not_null()
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    Table,
    Id,
    UserId,
    Email,
    Password,
    Username,
    Avatar,
}
