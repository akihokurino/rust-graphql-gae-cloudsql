pub mod application;
mod ddb;
pub mod domain;

#[macro_use]
extern crate diesel;

use diesel::{Connection, MysqlConnection};
use std::env;
use thiserror::Error as ThisErr;

#[derive(ThisErr, Debug, PartialOrd, PartialEq, Clone)]
pub enum AppError {
    #[error("不正なパラメーターです: {0}")]
    BadRequest(String),
    #[error("認証エラーです")]
    UnAuthenticate,
    #[error("禁止された行為です")]
    Forbidden,
    #[error("指定されたリソースが見つかりません")]
    NotFound,
    #[error("サーバーエラーです: {0}")]
    Internal(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl From<String> for AppError {
    fn from(v: String) -> Self {
        Self::Internal(v)
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => Self::NotFound,
            _ => Self::Internal(e.to_string()),
        }
    }
}

pub fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
