use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(WorkSet::Table)
                    .drop_column(WorkSet::TimeslotId)
                    .drop_column(WorkSet::TimeslotIndex)
                    .drop_column(WorkSet::Tempo)
                    .drop_column(WorkSet::Note)
                    .drop_column(WorkSet::SetType)
                    .add_column(integer(WorkSet::ExerciseId))
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_exercise_id")
                    .table(WorkSet::Table)
                    .col(WorkSet::ExerciseId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(WorkSet::Table)
                    .add_column(integer(WorkSet::TimeslotId))
                    .add_column(integer(WorkSet::TimeslotIndex))
                    .add_column(string(WorkSet::Tempo))
                    .add_column(string(WorkSet::Note))
                    .add_column(string(WorkSet::SetType))
                    .drop_column(WorkSet::ExerciseId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(WorkSet::Table)
                    .col(WorkSet::TimeslotId)
                    .name("idx_timeslot_id")
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum WorkSet {
    Table,
    ExerciseId,
    TimeslotId,
    TimeslotIndex,
    SetType,
    Tempo,
    Note,
}
