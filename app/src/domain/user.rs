use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct User {
    pub id: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl User {
    pub fn new(name: String, now: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            created_at: now.naive_utc(),
            updated_at: now.naive_utc(),
        }
    }

    pub fn update(&mut self, name: String, now: DateTime<Utc>) {
        self.name = name;
        self.updated_at = now.naive_utc();
    }
}
