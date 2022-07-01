use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set").to_owned();
    let connection = SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    connection
}

#[cfg(test)]
pub fn establish_connection_test() -> SqliteConnection {
    use diesel_migrations::run_pending_migrations;
    let connection = SqliteConnection::establish(":memory:")
        .expect("Error connecting to :memory:");
    run_pending_migrations(&connection).expect("failed to run migrations");
    connection
}