use sea_orm_migration::{
    prelude::*,
    schema::{date_time, integer, integer_null, pk_auto, string},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Timeslot::Table)
                    .if_not_exists()
                    .col(pk_auto(Timeslot::Id))
                    .col(integer(Timeslot::TrainerId))
                    .col(string(Timeslot::Name))
                    .col(date_time(Timeslot::Start))
                    .col(date_time(Timeslot::End))
                    .col(date_time(Timeslot::CreatedAt))
                    .col(date_time(Timeslot::UpdatedAt))
                    .col(integer_null(Timeslot::UserId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Timeslot::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Timeslot {
    Table,
    Id,
    TrainerId,
    UserId,
    Name,
    Start,
    End,
    UpdatedAt,
    CreatedAt,
}
