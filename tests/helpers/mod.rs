use diesel::SqliteConnection;
use diesel_migrations::run_pending_migrations;
use shoe_store::db::connect::establish_connection;

pub fn establish_connection_test() -> SqliteConnection {
    let test_database_url = ":memory:";
    let connection = establish_connection(test_database_url);
    run_pending_migrations(&connection).expect("failed to run migrations");
    connection
}
