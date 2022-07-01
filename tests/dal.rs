use diesel::Connection;
use shoe_store::db::{dal, models};
mod helpers;

#[test]
fn create_product_test() {
    use dal::{create_product, list_products};
    use diesel::result::Error;
    use helpers::establish_connection_test;
    use models::{
        NewCompleteProduct, NewProduct, NewVariant, NewVariantValue, Product, ProductVariant,
        Variant,
    };
    let connection = establish_connection_test();
    connection.test_transaction::<_, Error, _>(|| {
        let variant = NewVariant {
            name: "size".to_string(),
        };
        let variant_values = vec![
            Some(12.to_string()),
            Some(14.to_string()),
            Some(16.to_string()),
            Some(18.to_string()),
        ];
        let variants = vec![NewVariantValue {
            variant: variant.clone(),
            values: variant_values.clone(),
        }];

        create_product(
            NewCompleteProduct {
                product: NewProduct {
                    name: "boots".to_string(),
                    cost: 13.23,
                    active: true,
                },
                variants: variants.clone(),
            },
            &connection,
        )
        .unwrap();
        create_product(
            NewCompleteProduct {
                product: NewProduct {
                    name: "high heels".to_string(),
                    cost: 20.99,
                    active: true,
                },
                variants: variants.clone(),
            },
            &connection,
        )
        .unwrap();
        create_product(
            NewCompleteProduct {
                product: NewProduct {
                    name: "running shoes".to_string(),
                    cost: 10.99,
                    active: true,
                },
                variants: variants.clone(),
            },
            &connection,
        )
        .unwrap();

        let variants_result = |start_id: i32, for_product_id: i32| {
            vec![
                (
                    ProductVariant {
                        id: start_id + 1,
                        variant_id: 1,
                        product_id: for_product_id,
                        value: Some("12".to_string()),
                    },
                    Variant {
                        id: 1,
                        name: variant.name.to_string(),
                    },
                ),
                (
                    ProductVariant {
                        id: start_id + 2,
                        variant_id: 1,
                        product_id: for_product_id,
                        value: Some("14".to_string()),
                    },
                    Variant {
                        id: 1,
                        name: variant.name.to_string(),
                    },
                ),
                (
                    ProductVariant {
                        id: start_id + 3,
                        variant_id: 1,
                        product_id: for_product_id,
                        value: Some("16".to_string()),
                    },
                    Variant {
                        id: 1,
                        name: variant.name.to_string(),
                    },
                ),
                (
                    ProductVariant {
                        id: start_id + 4,
                        variant_id: 1,
                        product_id: for_product_id,
                        value: Some("18".to_string()),
                    },
                    Variant {
                        id: 1,
                        name: variant.name.to_string(),
                    },
                ),
            ]
        };

        assert_eq!(
            serde_json::to_string(&list_products(&connection).unwrap()).unwrap(),
            serde_json::to_string(&vec![
                (
                    Product {
                        id: 1,
                        name: "boots".to_string(),
                        cost: 13.23,
                        active: true
                    },
                    variants_result(0 * variant_values.len() as i32, 1)
                ),
                (
                    Product {
                        id: 2,
                        name: "high heels".to_string(),
                        cost: 20.99,
                        active: true
                    },
                    variants_result(1 * variant_values.len() as i32, 2)
                ),
                (
                    Product {
                        id: 3,
                        name: "running shoes".to_string(),
                        cost: 10.99,
                        active: true
                    },
                    variants_result(2 * variant_values.len() as i32, 3)
                )
            ])
            .unwrap()
        );

        Ok(())
    });
}

#[test]
fn show_product_test() {
    use dal::{create_product, show_product};
    use diesel::result::Error;
    use helpers::establish_connection_test;
    use models::{
        NewCompleteProduct, NewProduct, NewVariant, NewVariantValue, Product, ProductVariant,
        Variant,
    };
    let connection = establish_connection_test();
    connection.test_transaction::<_, Error, _>(|| {
        let product_id = create_product(
            NewCompleteProduct {
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
            },
            &connection,
        )
        .unwrap();

        assert_eq!(
            serde_json::to_string(&show_product(product_id, &connection).unwrap()).unwrap(),
            serde_json::to_string(&(
                Product {
                    id: 1,
                    name: "boots".to_string(),
                    cost: 13.23,
                    active: true
                },
                vec![
                    (
                        ProductVariant {
                            id: 1,
                            variant_id: 1,
                            product_id: 1,
                            value: Some("12".to_string()),
                        },
                        Variant {
                            id: 1,
                            name: "size".to_string()
                        }
                    ),
                    (
                        ProductVariant {
                            id: 2,
                            variant_id: 1,
                            product_id: 1,
                            value: Some("14".to_string()),
                        },
                        Variant {
                            id: 1,
                            name: "size".to_string()
                        }
                    ),
                    (
                        ProductVariant {
                            id: 3,
                            variant_id: 1,
                            product_id: 1,
                            value: Some("16".to_string()),
                        },
                        Variant {
                            id: 1,
                            name: "size".to_string()
                        }
                    ),
                    (
                        ProductVariant {
                            id: 4,
                            variant_id: 1,
                            product_id: 1,
                            value: Some("18".to_string()),
                        },
                        Variant {
                            id: 1,
                            name: "size".to_string()
                        }
                    )
                ]
            ))
            .unwrap()
        );

        Ok(())
    });
}

#[test]
fn search_products_test() {
    use dal::{create_product, search_products};
    use diesel::result::Error;
    use helpers::establish_connection_test;
    use models::{
        NewCompleteProduct, NewProduct, NewVariant, NewVariantValue, Product, ProductVariant,
        Variant,
    };
    let connection = establish_connection_test();
    connection.test_transaction::<_, Error, _>(|| {
        let variants = vec![NewVariantValue {
            variant: NewVariant {
                name: "size".to_string(),
            },
            values: vec![Some(12.to_string())],
        }];

        create_product(
            NewCompleteProduct {
                product: NewProduct {
                    name: "boots".to_string(),
                    cost: 13.23,
                    active: true,
                },
                variants: variants.clone(),
            },
            &connection,
        )
        .unwrap();
        create_product(
            NewCompleteProduct {
                product: NewProduct {
                    name: "high heels".to_string(),
                    cost: 20.99,
                    active: true,
                },
                variants: variants.clone(),
            },
            &connection,
        )
        .unwrap();
        create_product(
            NewCompleteProduct {
                product: NewProduct {
                    name: "running shoes".to_string(),
                    cost: 10.99,
                    active: true,
                },
                variants: variants.clone(),
            },
            &connection,
        )
        .unwrap();

        assert_eq!(
            serde_json::to_string(&search_products("shoes".to_string(), &connection).unwrap())
                .unwrap(),
            serde_json::to_string(&vec![(
                Product {
                    id: 3,
                    name: "running shoes".to_string(),
                    cost: 10.99,
                    active: true
                },
                vec![(
                    ProductVariant {
                        id: 3,
                        variant_id: 1,
                        product_id: 3,
                        value: Some("12".to_string()),
                    },
                    Variant {
                        id: 1,
                        name: "size".to_string(),
                    }
                )]
            )])
            .unwrap()
        );

        Ok(())
    });
}
