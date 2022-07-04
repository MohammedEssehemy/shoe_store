use diesel_migrations::run_pending_migrations;
use shoe_store::db::connect::{establish_connection, DbPool};

pub fn establish_connection_test() -> DbPool {
    let test_database_url = ":memory:";
    let pool = establish_connection(test_database_url);
    let conn = pool.get().unwrap();
    run_pending_migrations(&conn).expect("failed to run migrations");
    pool
}
