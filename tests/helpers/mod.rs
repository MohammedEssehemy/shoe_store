use diesel::{Connection, SqliteConnection};

pub fn establish_connection_test() -> SqliteConnection {
    use diesel_migrations::run_pending_migrations;
    let connection = SqliteConnection::establish(":memory:").expect("Error connecting to :memory:");
    run_pending_migrations(&connection).expect("failed to run migrations");
    connection
}
