use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WorkSet::Table)
                    .if_not_exists()
                    .col(pk_auto(WorkSet::Id))
                    .col(integer(WorkSet::TimeslotId))
                    .col(integer(WorkSet::TimeslotIndex))
                    .col(string(WorkSet::SetType))
                    .col(integer(WorkSet::Reps))
                    .col(string(WorkSet::Intensity))
                    .col(integer_null(WorkSet::Rpe))
                    .col(string_null(WorkSet::Tempo))
                    .col(string_null(WorkSet::Note))
                    .col(date_time(WorkSet::CreatedAt))
                    .col(date_time(WorkSet::UpdatedAt))
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_timeslot_id")
                    .table(WorkSet::Table)
                    .col(WorkSet::TimeslotId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WorkSet::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum WorkSet {
    Table,
    Id,
    TimeslotId,
    TimeslotIndex,
    SetType,
    Reps,
    Intensity,
    Rpe,
    Tempo,
    Note,
    CreatedAt,
    UpdatedAt,
}
