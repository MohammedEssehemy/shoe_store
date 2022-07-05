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

#[actix_web::test]
async fn test_product_list() {
    let pool = establish_connection_test();
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(actions::product_create)
            .service(actions::product_list),
    )
    .await;

    let shoes = vec![
        ("Boots", 14.00),
        ("High Heels", 19.23),
        ("Running Shoes", 21.90),
        ("Tennis Shoes", 15.67),
        ("Hiking Boots", 18.72),
        ("Flip Flops", 10.5),
    ];

    for shoe in shoes {
        let body = NewCompleteProduct {
            product: NewProduct {
                name: shoe.0.to_string(),
                cost: shoe.1,
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

    let req = test::TestRequest::get()
        .uri("/products?limit=5")
        .to_request();
    let result = test::call_and_read_body(&mut app, req).await;

    assert_eq!(
        web::Bytes::from_static(b"[[{\"id\":1,\"name\":\"Boots\",\"cost\":14.0,\"active\":true},[[{\"id\":1,\"product_id\":1,\"variant_id\":1,\"value\":\"12\"},{\"id\":1,\"name\":\"size\"}],[{\"id\":2,\"product_id\":1,\"variant_id\":1,\"value\":\"14\"},{\"id\":1,\"name\":\"size\"}],[{\"id\":3,\"product_id\":1,\"variant_id\":1,\"value\":\"16\"},{\"id\":1,\"name\":\"size\"}],[{\"id\":4,\"product_id\":1,\"variant_id\":1,\"value\":\"18\"},{\"id\":1,\"name\":\"size\"}]]],[{\"id\":2,\"name\":\"High Heels\",\"cost\":19.23,\"active\":true},[[{\"id\":5,\"product_id\":2,\"variant_id\":1,\"value\":\"12\"},{\"id\":1,\"name\":\"size\"}],[{\"id\":6,\"product_id\":2,\"variant_id\":1,\"value\":\"14\"},{\"id\":1,\"name\":\"size\"}],[{\"id\":7,\"product_id\":2,\"variant_id\":1,\"value\":\"16\"},{\"id\":1,\"name\":\"size\"}],[{\"id\":8,\"product_id\":2,\"variant_id\":1,\"value\":\"18\"},{\"id\":1,\"name\":\"size\"}]]],[{\"id\":3,\"name\":\"Running Shoes\",\"cost\":21.9,\"active\":true},[[{\"id\":9,\"product_id\":3,\"variant_id\":1,\"value\":\"12\"},{\"id\":1,\"name\":\"size\"}],[{\"id\":10,\"product_id\":3,\"variant_id\":1,\"value\":\"14\"},{\"id\":1,\"name\":\"size\"}],[{\"id\":11,\"product_id\":3,\"variant_id\":1,\"value\":\"16\"},{\"id\":1,\"name\":\"size\"}],[{\"id\":12,\"product_id\":3,\"variant_id\":1,\"value\":\"18\"},{\"id\":1,\"name\":\"size\"}]]],[{\"id\":4,\"name\":\"Tennis Shoes\",\"cost\":15.67,\"active\":true},[[{\"id\":13,\"product_id\":4,\"variant_id\":1,\"value\":\"12\"},{\"id\":1,\"name\":\"size\"}],[{\"id\":14,\"product_id\":4,\"variant_id\":1,\"value\":\"14\"},{\"id\":1,\"name\":\"size\"}],[{\"id\":15,\"product_id\":4,\"variant_id\":1,\"value\":\"16\"},{\"id\":1,\"name\":\"size\"}],[{\"id\":16,\"product_id\":4,\"variant_id\":1,\"value\":\"18\"},{\"id\":1,\"name\":\"size\"}]]],[{\"id\":5,\"name\":\"Hiking Boots\",\"cost\":18.72,\"active\":true},[[{\"id\":17,\"product_id\":5,\"variant_id\":1,\"value\":\"12\"},{\"id\":1,\"name\":\"size\"}],[{\"id\":18,\"product_id\":5,\"variant_id\":1,\"value\":\"14\"},{\"id\":1,\"name\":\"size\"}],[{\"id\":19,\"product_id\":5,\"variant_id\":1,\"value\":\"16\"},{\"id\":1,\"name\":\"size\"}],[{\"id\":20,\"product_id\":5,\"variant_id\":1,\"value\":\"18\"},{\"id\":1,\"name\":\"size\"}]]]]"),
       result,
      );
}

#[actix_web::test]
async fn test_product_show() {
    let pool = establish_connection_test();
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(actions::product_create)
            .service(actions::product_show),
    )
    .await;

    let body = NewCompleteProduct {
        product: NewProduct {
            name: "Boots".to_string(),
            cost: 15.69,
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

    let req = test::TestRequest::get().uri("/products/1").to_request();
    let resp = test::call_and_read_body(&mut app, req).await;

    assert_eq!(
            web::Bytes::from_static(
                b"[{\"id\":1,\"name\":\"Boots\",\"cost\":15.69,\"active\":true},[[{\"id\":1,\"product_id\":1,\"variant_id\":1,\"value\":\"12\"},{\"id\":1,\"name\":\"size\"}],[{\"id\":2,\"product_id\":1,\"variant_id\":1,\"value\":\"14\"},{\"id\":1,\"name\":\"size\"}],[{\"id\":3,\"product_id\":1,\"variant_id\":1,\"value\":\"16\"},{\"id\":1,\"name\":\"size\"}],[{\"id\":4,\"product_id\":1,\"variant_id\":1,\"value\":\"18\"},{\"id\":1,\"name\":\"size\"}]]]"
            ),
            resp
        );
}
