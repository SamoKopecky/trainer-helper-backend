pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250131_160240_add_indexes;
mod m20250208_120944_add_set_table;
mod m20250217_102250_add_exercise_table;
mod m20250217_103402_truncate_work_set_table;
mod m20250313_181110_add_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250131_160240_add_indexes::Migration),
            Box::new(m20250208_120944_add_set_table::Migration),
            Box::new(m20250217_102250_add_exercise_table::Migration),
            Box::new(m20250217_103402_truncate_work_set_table::Migration),
            Box::new(m20250313_181110_add_user_table::Migration),
        ]
    }
}
