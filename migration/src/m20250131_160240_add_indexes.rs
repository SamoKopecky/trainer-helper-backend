use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_table::Timeslot;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("idx_trainer_id")
                    .table(Timeslot::Table)
                    .col(Timeslot::TrainerId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_user_id")
                    .table(Timeslot::Table)
                    .col(Timeslot::UserId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_start")
                    .table(Timeslot::Table)
                    .col(Timeslot::Start)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_trainer_id")
                    .table(Timeslot::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_user_id")
                    .table(Timeslot::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_start")
                    .table(Timeslot::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
