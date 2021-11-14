use crate::graph::inputs::{CreateUserInput, UpdateUserInput};
use crate::graph::outputs::User;
use crate::graph::Context;
use crate::graph::FieldErrorWithCode;
use juniper::FieldResult;

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    async fn create_user(context: &Context, input: CreateUserInput) -> FieldResult<User> {
        let user = context
            .user_application
            .create(input.name)
            .map_err(FieldErrorWithCode::from)?;

        Ok(User {
            id: user.id.to_owned(),
            name: user.name.to_owned(),
        })
    }

    async fn update_user(context: &Context, input: UpdateUserInput) -> FieldResult<User> {
        let user = context
            .user_application
            .update(input.id, input.name)
            .map_err(FieldErrorWithCode::from)?;

        Ok(User {
            id: user.id.to_owned(),
            name: user.name.to_owned(),
        })
    }

    async fn delete_user(context: &Context, id: String) -> FieldResult<bool> {
        context
            .user_application
            .delete(id)
            .map_err(FieldErrorWithCode::from)?;

        Ok(true)
    }
}
