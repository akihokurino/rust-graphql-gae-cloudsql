use crate::ddb::schema::users;
use crate::ddb::Dao;
use crate::domain;
use crate::{AppError, AppResult};
use diesel::prelude::*;
use std::convert::TryFrom;

#[derive(Queryable, Insertable, Debug, Clone, Eq, PartialEq, Identifiable, AsChangeset)]
#[table_name = "users"]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl TryFrom<&Entity> for domain::user::User {
    type Error = String;

    fn try_from(e: &Entity) -> Result<Self, Self::Error> {
        Ok(domain::user::User {
            id: e.id.to_owned(),
            name: e.name.to_owned(),
            created_at: e.created_at.to_owned(),
            updated_at: e.updated_at.to_owned(),
        })
    }
}

impl From<&domain::user::User> for Entity {
    fn from(d: &domain::user::User) -> Entity {
        Entity {
            id: d.id.to_owned(),
            name: d.name.to_owned(),
            created_at: d.created_at.to_owned(),
            updated_at: d.updated_at.to_owned(),
        }
    }
}

impl Dao<domain::user::User> {
    pub fn get_all(&self, conn: &MysqlConnection) -> AppResult<Vec<domain::user::User>> {
        let user_entities: Vec<Entity> = users::table
            .order(users::created_at.desc())
            .load::<Entity>(conn)
            .map_err(AppError::from)?;

        Ok(user_entities
            .iter()
            .map(|v| domain::user::User::try_from(v).unwrap())
            .collect())
    }

    pub fn get(&self, conn: &MysqlConnection, id: String) -> AppResult<domain::user::User> {
        users::table
            .find(id)
            .first(conn)
            .map(|v: Entity| domain::user::User::try_from(&v).unwrap())
            .map_err(AppError::from)
    }

    pub fn insert(&self, conn: &MysqlConnection, item: &domain::user::User) -> AppResult<()> {
        let e: Entity = item.clone().into();
        if let Err(e) = diesel::insert_into(users::table)
            .values(e)
            .execute(conn)
            .map_err(AppError::from)
        {
            return Err(e);
        }
        Ok(())
    }

    pub fn update(&self, conn: &MysqlConnection, item: &domain::user::User) -> AppResult<()> {
        let e: Entity = item.clone().into();
        if let Err(e) = diesel::update(users::table.find(e.id.clone()))
            .set(&e)
            .execute(conn)
            .map_err(AppError::from)
        {
            return Err(e);
        }
        Ok(())
    }

    pub fn delete(&self, conn: &MysqlConnection, id: String) -> AppResult<()> {
        if let Err(e) = diesel::delete(users::table.find(id))
            .execute(conn)
            .map_err(AppError::from)
        {
            return Err(e);
        }
        Ok(())
    }
}
