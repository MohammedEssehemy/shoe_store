use actix_web::{test, web, App};
use helpers::establish_connection_test;
use models::{NewCompleteProduct, NewProduct, NewVariant, NewVariantValue};
use shoe_store::{actions, db::models};
mod helpers;

#[actix_web::test]
async fn test_product_creation_is_ok() {
    let pool = establish_connection_test();
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(actions::product_create),
    )
    .await;

    let body = NewCompleteProduct {
        product: NewProduct {
            name: "boots".to_string(),
            cost: 13.23,
            active: true,
        },
        variants: vec![NewVariantValue {
            variant: NewVariant {
                name: "size".to_string(),
            },
            values: vec![
                Some(12.to_string()),
                Some(14.to_string()),
                Some(16.to_string()),
                Some(18.to_string()),
            ],
        }],
    };

    let req = test::TestRequest::post()
        .set_json(&body)
        .uri("/products")
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}
