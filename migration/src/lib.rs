pub use sea_orm_migration::prelude::*;

mod m20220101_000001_user;
mod m20240210_025537_posts;
mod m20240228_162905_categorybook;
mod m20240228_163624_books;
 

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_user::Migration),
            Box::new(m20240210_025537_posts::Migration),
            Box::new(m20240228_162905_categorybook::Migration),
            Box::new(m20240228_163624_books::Migration),
        ]
    }
}
