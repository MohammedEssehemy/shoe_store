use actix_web::{get, post, web, Responder,Result, error::ErrorInternalServerError, HttpResponse};
use super::db::{
    connect::DbPool,
    dal::{create_product, list_products},
    models::NewCompleteProduct
};

#[post("/products")]
async fn product_create(product: web::Json<NewCompleteProduct>, pool: web::Data<DbPool>) -> Result<impl Responder> {
    let connection = pool.get().unwrap();
	match create_product(product.clone(), &connection) {
		Ok(_) => Ok(HttpResponse::Ok()),
		Err(error) => Err(ErrorInternalServerError(error))
	}
}

#[get("/products")]
async fn product_list(pool: web::Data<DbPool>) -> Result<impl Responder> {
    let connection = pool.get().unwrap();
	match list_products(&connection) {
		Ok(products) => Ok(HttpResponse::Ok().json(products)),
		Err(error) => Err(ErrorInternalServerError(error))
	}
}