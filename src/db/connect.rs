use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection(database_url: &str) -> Pool<ConnectionManager<SqliteConnection>> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("Error creating database pool");
    let conn = pool.get().expect("Error getting connection from pool");
    // Foreign key constraint is not enabled by default in SQLite
    // https://www.sqlite.org/foreignkeys.html
    conn.execute("PRAGMA foreign_keys = ON").unwrap();
    pool
}
