use actix_web::{get, web, App, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use shoe_store::db::{connect::establish_connection, dal::list_products};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or("8080".to_owned()).parse::<u16>().unwrap();
    let address = "127.0.0.1";
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = establish_connection(&database_url);
    let _products = list_products(&conn).unwrap();
    // println!("{:#?}", products);
    println!("server starting on {}:{}", address, port);
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
    })
    .bind((address, port))?
    .run()
    .await
}