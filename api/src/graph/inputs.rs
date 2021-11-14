#[derive(Debug, GraphQLInputObject)]
pub struct CreateUserInput {
    pub name: String,
}

#[derive(Debug, GraphQLInputObject)]
pub struct UpdateUserInput {
    pub id: String,
    pub name: String,
}
