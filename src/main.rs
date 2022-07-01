use shoe_store::db::{connect::establish_connection, dal::list_products};

fn main() {
    let conn = establish_connection();
    let products = list_products(&conn).unwrap();
    println!("{:#?}", products);
}
