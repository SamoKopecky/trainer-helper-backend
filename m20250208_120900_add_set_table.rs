use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(aSet::Table)
                    .if_not_exists()
                    .col(pk_auto(Set::Id))
                    .col(string(Set::Title))
                    .col(string(Set::Text))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table(Set::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Set{
    Table,
    Id,
    TimeslotId,
    TimeslotIndex,
    Type,
    Reps,
    Intesity,
    Rpe,
    Tempo,
    Note,
}
