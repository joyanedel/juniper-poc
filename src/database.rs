use diesel::{Connection, SqliteConnection};

pub fn establish_connection(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error at connecting to database {}", database_url))
}
