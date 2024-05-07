use diesel::{deserialize::Queryable, prelude::Insertable, Selectable};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: Option<String>,
    pub name: String,
    pub email: String,
    pub hashed_password: String,
    pub created_at: Option<String>,
}
