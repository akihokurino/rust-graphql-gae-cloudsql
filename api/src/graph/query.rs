use crate::graph::outputs::User;
use crate::graph::Context;
use crate::graph::FieldErrorWithCode;
use juniper::FieldResult;

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    async fn users(context: &Context) -> FieldResult<Vec<User>> {
        let users = context
            .user_application
            .list()
            .map_err(FieldErrorWithCode::from)?;

        Ok(users
            .iter()
            .map(|v| User {
                id: v.id.to_owned(),
                name: v.name.to_owned(),
            })
            .collect())
    }

    async fn user(context: &Context, id: String) -> FieldResult<User> {
        let user = context
            .user_application
            .get(id)
            .map_err(FieldErrorWithCode::from)?;

        Ok(User {
            id: user.id.to_owned(),
            name: user.name.to_owned(),
        })
    }
}
