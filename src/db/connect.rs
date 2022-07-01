use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub fn establish_connection(database_url: &str) -> SqliteConnection {
    let connection = SqliteConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url));
    // Foreign key constraint is not enabled by default in SQLite
    // https://www.sqlite.org/foreignkeys.html
    connection.execute("PRAGMA foreign_keys = ON").unwrap();
    connection
}
