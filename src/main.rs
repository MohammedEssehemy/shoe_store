use dotenv::dotenv;
use std::env;
use shoe_store::db::{connect::establish_connection, dal::list_products};

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = establish_connection(&database_url);
    let products = list_products(&conn).unwrap();
    println!("{:#?}", products);
}
