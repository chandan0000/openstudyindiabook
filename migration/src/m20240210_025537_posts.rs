use sea_orm_migration::prelude::*;

use crate::m20220101_000001_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Posts::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Posts::OwnerId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("owner_id")
                            .from(Posts::Table, Posts::OwnerId)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(Posts::Title).string().not_null())
                    .col(ColumnDef::new(Posts::Text).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    UserId,
    Id,
    Title,
    Text,
    OwnerId,
}
