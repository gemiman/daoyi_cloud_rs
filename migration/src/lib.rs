pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;

pub struct Migrator;

#[derive(DeriveIden)]
enum InfraMigrationTable {
    #[sea_orm(table_name = "seaql_migrations", schema_name = "infra")]
    Table,
}

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration)]
    }

    fn migration_table_name() -> sea_query::DynIden {
        InfraMigrationTable::Table.into_iden()
    }
}
