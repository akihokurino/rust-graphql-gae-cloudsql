mod inputs;
mod mutation;
mod outputs;
mod query;

use crate::graph::mutation::MutationRoot;
use crate::graph::query::QueryRoot;
use app::application::*;
use app::AppError;
use convert_case::{Case, Casing};
use juniper::{EmptySubscription, FieldError, RootNode};
use std::sync::{Arc, Mutex};
use strum_macros::Display as StrumDisplay;

pub struct Context {
    pub user_application: user::Application,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new() -> Self {
        let conn_ref = Arc::new(Mutex::new(app::establish_connection()));
        let user_application = user::Application::new(conn_ref);

        Self { user_application }
    }
}

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

#[derive(StrumDisplay, Debug)]
pub enum FieldErrorCode {
    BadRequest,
    UnAuthenticate,
    NotFound,
    Forbidden,
    Internal,
}

pub struct FieldErrorWithCode {
    err: AppError,
    code: FieldErrorCode,
}

impl From<AppError> for FieldErrorWithCode {
    fn from(err: AppError) -> Self {
        FieldErrorWithCode {
            err: err.clone(),
            code: match err {
                AppError::BadRequest(_) => FieldErrorCode::BadRequest,
                AppError::UnAuthenticate => FieldErrorCode::UnAuthenticate,
                AppError::Forbidden => FieldErrorCode::Forbidden,
                AppError::NotFound => FieldErrorCode::NotFound,
                AppError::Internal(_) => FieldErrorCode::Internal,
            },
        }
    }
}

impl From<FieldErrorWithCode> for FieldError {
    fn from(v: FieldErrorWithCode) -> Self {
        let code = v.code.to_string().to_case(Case::UpperSnake);

        FieldError::new(
            v.err,
            graphql_value!({
                "code": code,
            }),
        )
    }
}
