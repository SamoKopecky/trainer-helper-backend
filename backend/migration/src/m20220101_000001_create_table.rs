use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(Timeslot::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Timeslot::TrainerId).integer().not_null())
                    .col(ColumnDef::new(Timeslot::Start).date_time().not_null())
                    .col(ColumnDef::new(Timeslot::Duration).integer().not_null())
                    .col(ColumnDef::new(Timeslot::UpdatedAt).date_time().not_null())
                    .col(ColumnDef::new(Timeslot::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Timeslot::UserId).integer().null())
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
    Start,
    Duration,
    UpdatedAt,
    CreatedAt,
}
