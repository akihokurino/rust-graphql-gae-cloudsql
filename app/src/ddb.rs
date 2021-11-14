mod schema;
mod user;

use crate::AppResult;
use diesel::connection::TransactionManager;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use std::env;
use std::future::Future;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Dao<T> {
    _phantom: PhantomData<fn() -> T>,
}

impl<T> Dao<T> {
    pub fn new() -> Self {
        Dao {
            _phantom: PhantomData,
        }
    }
}

pub struct Tx {}

impl Tx {
    pub fn run<R, F>(conn: &MysqlConnection, f: F) -> AppResult<R>
    where
        F: FnOnce() -> AppResult<R>,
    {
        conn.transaction(|| f())
    }

    pub async fn run_async<R, F>(conn: &MysqlConnection, f: F) -> AppResult<R>
    where
        F: Future<Output = AppResult<R>>,
    {
        let transaction_manager = conn.transaction_manager();
        transaction_manager.begin_transaction(conn)?;
        match f.await {
            Ok(value) => {
                transaction_manager.commit_transaction(conn)?;
                Ok(value)
            }
            Err(e) => {
                transaction_manager.rollback_transaction(conn)?;
                Err(e)
            }
        }
    }
}
