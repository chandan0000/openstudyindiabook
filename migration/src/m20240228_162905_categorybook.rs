use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CategoryBook::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CategoryBook::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CategoryBook::Title).string().unique_key().not_null())
                    .col(
                        ColumnDef::new(CategoryBook::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()), // Use default_expr for expressions
                    )
                    .col(
                        ColumnDef::new(CategoryBook::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CategoryBook::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum CategoryBook {
    Table,
    Id,
    Title,

    CreatedAt,
    UpdatedAt,
}
