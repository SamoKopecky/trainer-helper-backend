use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Exercise::Table)
                    .if_not_exists()
                    .col(pk_auto(Exercise::Id))
                    .col(integer(Exercise::TimeslotId))
                    .col(integer(Exercise::GroupId))
                    .col(string(Exercise::SetType))
                    .col(string(Exercise::Note))
                    .col(date_time(Exercise::CreatedAt))
                    .col(date_time(Exercise::UpdatedAt))
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_exercise_timeslot_id")
                    .table(Exercise::Table)
                    .col(Exercise::TimeslotId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_group_id")
                    .table(Exercise::Table)
                    .col(Exercise::GroupId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Exercise::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Exercise {
    Table,
    Id,
    TimeslotId,
    GroupId,
    SetType,
    Note,
    CreatedAt,
    UpdatedAt,
}
