use actix_web::{get, post, web, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use super::db::{
    connect::DbPool,
    dal::{create_product, list_products},
    models::NewCompleteProduct
};

#[post("/products")]
async fn product_create(product: web::Json<NewCompleteProduct>, pool: web::Data<DbPool>) -> impl Responder {
    let connection = pool.get().unwrap();
	let product = product.into_inner();
	let _create_product_result = web::block(move || create_product(product, &connection).unwrap())
	.await
	.map_err(|e| {
            log::error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
	.unwrap();
	HttpResponse::Created()
}

#[derive(Serialize, Deserialize)]
struct ProductListQueryParams {
	limit: Option<u16>,
}

#[get("/products")]
async fn product_list(query_params: web::Query<ProductListQueryParams>, pool: web::Data<DbPool>) -> impl Responder {
	let connection = pool.get().unwrap();
	let products = web::block(move || list_products(query_params.limit, &connection).unwrap())
	.await
	.map_err(|e| {
            log::error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
	.unwrap();
	HttpResponse::Ok().json(products)
}