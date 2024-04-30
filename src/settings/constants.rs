use crate::settings::env::get_env_var_or_default;

#[derive(Debug)]
pub struct Constants {
    pub database_url: String,
}

impl Constants {
    pub fn new() -> Self {
        Constants {
            database_url: get_env_var_or_default("DATABASE_URL", "sqlite://db.sqlite3".to_string()),
        }
    }
}
