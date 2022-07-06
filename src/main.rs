use actix_web::{middleware::Logger, web, App, http, HttpServer, cookie::Key};
use actix_cors::Cors;
use actix_session::{
    SessionMiddleware,
    storage::CookieSessionStore
};
use dotenv::dotenv;
use std::env;
use env_logger::Env;
use shoe_store::{
    db::connect::establish_connection,
    actions
};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or("8080".to_owned()).parse::<u16>().unwrap();
    let address = "127.0.0.1";
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = establish_connection(&database_url);
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    log::info!("starting HTTP server at {address}:{port}");
    HttpServer::new(move || {
        let cors_mw = Cors::default()
            .allowed_origin(env::var("CORS_ALLOWED_ORIGINS").unwrap_or("https://my_domain.com/".to_owned()).as_ref())
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        let secret_key = Key::generate();
        let session_mw = SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
        .cookie_secure(false)
        .build();
        App::new()
            .wrap(session_mw)
            .wrap(cors_mw)
            .app_data(web::Data::new(conn.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(actions::product_create)
            .service(actions::product_list)
            .service(actions::product_search)
            .service(actions::product_show)
            .service(actions::product_update)
            .service(actions::product_delete)
    })
    .bind((address, port))?
    .run()
    .await
}