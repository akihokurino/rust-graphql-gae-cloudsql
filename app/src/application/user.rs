use crate::ddb::{Dao, Tx};
use crate::domain::user::User;
use crate::AppResult;
use chrono::{DateTime, Utc};
use diesel::MysqlConnection;
use std::sync::{Arc, Mutex};

pub struct Application {
    user_dao: Dao<User>,
    db_conn: Arc<Mutex<MysqlConnection>>,
}

impl Application {
    pub fn new(db_conn: Arc<Mutex<MysqlConnection>>) -> Self {
        let user_dao: Dao<User> = Dao::new();
        Self { user_dao, db_conn }
    }

    pub fn list(&self) -> AppResult<Vec<User>> {
        let conn = self.db_conn.lock().unwrap();
        let users = self.user_dao.get_all(&conn)?;
        Ok(users)
    }

    pub fn get(&self, id: String) -> AppResult<User> {
        let conn = self.db_conn.lock().unwrap();
        let user = self.user_dao.get(&conn, id)?;
        Ok(user)
    }

    pub fn create(&self, name: String) -> AppResult<User> {
        let conn = self.db_conn.lock().unwrap();
        let now: DateTime<Utc> = Utc::now();
        let user = User::new(name, now);
        self.user_dao.insert(&conn, &user)?;
        Ok(user)
    }

    pub fn update(&self, id: String, name: String) -> AppResult<User> {
        let conn = self.db_conn.lock().unwrap();
        let now: DateTime<Utc> = Utc::now();

        let user = Tx::run(&conn, || {
            let mut user = self.user_dao.get(&conn, id)?;
            user.update(name, now);
            self.user_dao.update(&conn, &user)?;
            Ok(user)
        })?;

        Ok(user)
    }

    pub fn delete(&self, id: String) -> AppResult<()> {
        let conn = self.db_conn.lock().unwrap();
        self.user_dao.delete(&conn, id)?;
        Ok(())
    }
}
