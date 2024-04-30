use juniper::{EmptySubscription, FieldResult, GraphQLObject, RootNode};

#[derive(GraphQLObject)]
#[graphql(description = "User object")]
struct User {
    id: i32,
    name: String,
    hashed_password: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn user() -> FieldResult<User> {
        Ok(User {
            id: 3,
            name: "Jesús".to_owned(),
            hashed_password: "sdas".to_owned(),
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_user(name: String, password: String) -> FieldResult<User> {
        Ok(User {
            id: 3,
            name,
            hashed_password: password,
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
