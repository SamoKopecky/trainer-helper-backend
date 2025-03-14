use chrono::Utc;
use entity::{person, timeslot};
use sea_orm::{entity::prelude::*, Set};

use super::{models::timeslot::PersonTimeslot, ResultCRUD};

pub struct CRUDTimeslot;

impl CRUDTimeslot {
    pub async fn get_by_range_date(
        db_conn: &DatabaseConnection,
        start_date: DateTime,
        end_date: DateTime,
    ) -> ResultCRUD<Vec<PersonTimeslot>> {
        let res = timeslot::Entity::find()
            .filter(timeslot::Column::Start.between(start_date, end_date))
            .find_also_related(person::Entity)
            .all(db_conn)
            .await?;

        Ok(res
            .into_iter()
            .map(|(timeslot, maybe_person)| {
                if let Some(person) = maybe_person {
                    PersonTimeslot {
                        timeslot,
                        person_name: Some(person.name),
                    }
                } else {
                    PersonTimeslot {
                        timeslot,
                        person_name: None,
                    }
                }
            })
            .collect())
    }

    pub async fn get_by_id(db_conn: &DatabaseConnection, id: i32) -> ResultCRUD<timeslot::Model> {
        Ok(timeslot::Entity::find_by_id(id)
            .one(db_conn)
            .await?
            .unwrap())
    }

    pub async fn insert_timeslot(
        db_conn: &DatabaseConnection,
        new_timeslot: timeslot::ActiveModel,
    ) -> ResultCRUD<timeslot::Model> {
        Ok(new_timeslot.insert(db_conn).await?)
    }

    pub async fn delete_timeslot(
        db_conn: &DatabaseConnection,
        timeslot_id: i32,
    ) -> ResultCRUD<timeslot::Model> {
        let to_delete = timeslot::Entity::find_by_id(timeslot_id)
            .one(db_conn)
            .await
            .unwrap();
        if let Some(to_delete) = to_delete {
            to_delete.clone().delete(db_conn).await.unwrap();
            Ok(to_delete)
        } else {
            Err(DbErr::RecordNotFound("Timeslot not found".to_string()))
        }
    }

    pub async fn update_by_id(
        db_conn: &DatabaseConnection,
        id: i32,
        mut data: timeslot::ActiveModel,
    ) -> ResultCRUD<timeslot::Model> {
        data.id = Set(id);
        data.updated_at = Set(Utc::now().naive_local());
        data.update(db_conn).await
    }
}
