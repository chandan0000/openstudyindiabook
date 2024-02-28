use crate::m20240228_162905_categorybook::CategoryBook;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Books::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Books::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Books::Title).string().not_null())
                    .col(ColumnDef::new(Books::Summery).string().not_null())
                    .col(ColumnDef::new(Books::PdfUrl).string().not_null())
                    .col(ColumnDef::new(Books::CategoryId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("category_id")
                            .from(Books::Table, Books::CategoryId)
                            .to(CategoryBook::Table, CategoryBook::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Books::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()), // Use default_expr for expressions
                    )
                    .col(
                        ColumnDef::new(Books::UpdatedAt)
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
            .drop_table(Table::drop().table(Books::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Books {
    Table,
    Id,
    Title,
    Summery,
    PdfUrl,
    CategoryId,
    CreatedAt,
    UpdatedAt,
}
