use diesel::associations::HasTable;
use diesel::query_dsl::methods::{LimitDsl, SelectDsl};
use diesel::{RunQueryDsl, SelectableHelper};
use juniper::{EmptySubscription, FieldResult, GraphQLObject, RootNode};
use uuid::Uuid;

use crate::database::establish_connection;
use crate::models::user::User as SQLUser;
use crate::schema::users::dsl::users;
use crate::settings::constants::Constants;

#[derive(GraphQLObject)]
#[graphql(description = "User object")]
struct User {
    id: String,
    name: String,
    hashed_password: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn users() -> FieldResult<Vec<User>> {
        let vars = Constants::new();
        let mut db_conn = establish_connection(vars.database_url.as_str());
        let users_result: Vec<SQLUser> = users
            .limit(5)
            .select(SQLUser::as_select())
            .load(&mut db_conn)
            .expect("Couldn't get user");
        let users_result = users_result
            .into_iter()
            .map(|el| User {
                id: el.id.unwrap(),
                name: el.name,
                hashed_password: el.hashed_password,
            })
            .collect();

        Ok(users_result)
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_user(name: String, email: String, password: String) -> FieldResult<User> {
        let vars = Constants::new();
        let mut db_conn = establish_connection(vars.database_url.as_str());

        let user_id = Uuid::new_v4();
        let hashed_password = password;

        let new_user = SQLUser {
            id: Some(user_id.into()),
            name,
            email,
            hashed_password,
            created_at: None,
        };

        let _ = diesel::insert_into(users::table())
            .values(&new_user)
            .execute(&mut db_conn);

        Ok(User {
            id: user_id.into(),
            name: new_user.name,
            hashed_password: new_user.hashed_password,
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
