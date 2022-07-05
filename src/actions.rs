use actix_web::{get, post, web, Responder, HttpResponse, put, delete};
use serde::{Serialize, Deserialize};
use crate::db::dal::{update_product, delete_product};

use super::db::{
    connect::DbPool,
    dal::{create_product, search_products, list_products, show_product},
    models::{FormProduct, NewCompleteProduct}
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
	let ProductListQueryParams { limit } = query_params.into_inner();
	let products = web::block(move || list_products(limit, &connection).unwrap())
	.await
	.map_err(|e| {
            log::error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
	.unwrap();
	HttpResponse::Ok().json(products)
}


#[derive(Serialize, Deserialize)]
struct ProductSearchQueryParams {
	search: String
}

#[get("/products/search")]
async fn product_search(query: web::Query<ProductSearchQueryParams>, pool: web::Data<DbPool>) -> impl Responder {
	let connection = pool.get().unwrap();
	let ProductSearchQueryParams { search } = query.into_inner();
	let products = web::block(move || search_products(search, &connection).unwrap())
	.await
	.map_err(|e| {
            log::error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
	.unwrap();
	HttpResponse::Ok().json(products)
}

#[get("/products/{id}")]
async fn product_show(id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
	let id = id.into_inner();
	let connection = pool.get().unwrap();
	let product = web::block(move || show_product(id, &connection).unwrap())
	.await
	.map_err(|e| {
            log::error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
	.unwrap();
	HttpResponse::Ok().json(product)
}


#[put("/products/{id}")]
async fn product_update(id: web::Path<i32>, product: web::Json<FormProduct>, pool: web::Data<DbPool>) -> impl Responder {
    let connection = pool.get().unwrap();
	let id = id.into_inner();
	let product = product.into_inner();
	let _update_product_result = web::block(move || update_product(id, product, &connection).unwrap())
	.await
	.map_err(|e| {
            log::error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
	.unwrap();
	HttpResponse::Ok()
}


#[delete("/products/{id}")]
async fn product_delete(id: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
	let connection = pool.get().unwrap();
	let id = id.into_inner();
	let _delete_product_result = web::block(move || delete_product(id, &connection).unwrap())
	.await
	.map_err(|e| {
            log::error!("{}", e);
            HttpResponse::InternalServerError().finish()
        })
	.unwrap();
	HttpResponse::Ok()
}