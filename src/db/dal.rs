use anyhow::Result;
use diesel::{
    sqlite::SqliteConnection,
    ExpressionMethods,
    TextExpressionMethods,
    Connection,
    QueryDsl, 
    RunQueryDsl, 
    BelongingToDsl,
    GroupedBy, 
};
use super::models::{
    Product,
    Variant,
    ProductVariant,
    NewCompleteProduct, 
};
use super::schema::{
    products,
    variants,
    products_variants,
};

no_arg_sql_function!(last_insert_rowid, diesel::sql_types::Integer);

pub fn create_product(new_product: NewCompleteProduct, conn: &SqliteConnection) -> Result<i32>  {
    conn.transaction(|| {
        diesel::insert_into(products::table)
            .values(new_product.product)
            .execute(conn)?;

        let last_product_id = diesel::select(last_insert_rowid).first(conn)?;

        for new_variant in new_product.variants {
            let variants_result =
                variants::table
                    .filter(variants::name.eq(&new_variant.variant.name))
                    .limit(1)
                    .load::<Variant>(conn)?;

            let last_variant_id =
                match variants_result.first() {
                    Some(variant) => variant.id,
                    None => {
                        diesel::insert_into(variants::table)
                            .values(variants::name.eq(&new_variant.variant.name))
                            .execute(conn)?;

                        diesel::select(last_insert_rowid).first(conn)?
                    }
                };

            for new_value in new_variant.values {
                diesel::insert_into(products_variants::table)
                    .values(
                        (
                            products_variants::product_id.eq(last_product_id), 
                            products_variants::variant_id.eq(last_variant_id),
                            products_variants::value.eq(new_value), 
                        )
                    ).execute(conn)?;
            }
        }
        Ok(last_product_id)
    })
}

fn show_product(id: i32, conn: &SqliteConnection) -> Result<(Product, Vec<(ProductVariant, Variant)>)> {
    let product_result =
        products::table
            .find(id)
            .get_result::<Product>(conn)?;

    let variants_result =
        ProductVariant::belonging_to(&product_result)
            .inner_join(variants::table)
            .load::<(ProductVariant, Variant)>(conn)?;

    Ok((product_result, variants_result))
}


pub fn list_products(conn: &SqliteConnection) ->  Result<Vec<(Product, Vec<(ProductVariant, Variant)>)>> {
 let products_result = 
        products::table
        .load::<Product>(conn)?;
    let variants_result =
        ProductVariant::belonging_to(&products_result)
            .inner_join(variants::table)
            .load::<(ProductVariant, Variant)>(conn)?
            .grouped_by(&products_result);
    let data = products_result.into_iter().zip(variants_result).collect::<Vec<_>>();

    Ok(data)
}

fn search_products(search: String, conn: &SqliteConnection) -> Result<Vec<(Product, Vec<(ProductVariant, Variant)>)>> {
    let pattern = format!("%{}%", search);
    let products_result = 
        products::table
        .filter(products::name.like(pattern))
        .load::<Product>(conn)?;
    let variants_result =
        ProductVariant::belonging_to(&products_result)
            .inner_join(variants::table)
            .load::<(ProductVariant, Variant)>(conn)?
            .grouped_by(&products_result);
    let data = products_result.into_iter().zip(variants_result).collect::<Vec<_>>();

    Ok(data)
}

#[test]
fn create_product_test() {
    use diesel::result::Error;
    use super::connect::establish_connection_test;
    use super::models::{ NewCompleteProduct, NewVariant, NewVariantValue, NewProduct, Product, Variant, ProductVariant };
    let connection = establish_connection_test();
    connection.test_transaction::<_, Error, _>(|| {
        let variant = NewVariant { name: "size".to_string() };
        let variant_values = vec![
                    Some(12.to_string()),
                    Some(14.to_string()),
                    Some(16.to_string()),
                    Some(18.to_string())
                ];
        let variants = vec![
            NewVariantValue {
                variant: variant.clone(),
                values: variant_values.clone()
            }
        ];

        create_product(NewCompleteProduct {
            product: NewProduct { name: "boots".to_string(), cost: 13.23, active: true },
            variants: variants.clone()
        }, &connection).unwrap();
        create_product(NewCompleteProduct {
            product: NewProduct { name: "high heels".to_string(), cost: 20.99, active: true },
            variants: variants.clone()
        }, &connection).unwrap();
        create_product(NewCompleteProduct {
            product: NewProduct { name: "running shoes".to_string(), cost: 10.99, active: true },
            variants: variants.clone()
        }, &connection).unwrap();


     let variants_result = |start_id: i32, for_product_id: i32| {
            vec![
                (
                    ProductVariant {
                        id: start_id + 1,
                        variant_id: 1,
                        product_id: for_product_id,
                        value: Some("12".to_string()),
                    },
                    Variant { id: 1, name: variant.name.to_string() },
                ),
                (
                    ProductVariant {
                        id: start_id + 2,
                        variant_id: 1,
                        product_id: for_product_id,
                        value: Some("14".to_string()),
                    },
                    Variant { id: 1, name: variant.name.to_string() },
                ),
                (
                    ProductVariant {
                        id: start_id + 3,
                        variant_id: 1,
                        product_id: for_product_id,
                        value: Some("16".to_string()),
                    },
                    Variant { id: 1, name: variant.name.to_string() },
                ),
                (
                    ProductVariant {
                        id: start_id + 4,
                        variant_id: 1,
                        product_id: for_product_id,
                        value: Some("18".to_string()),
                    },
                    Variant { id: 1, name: variant.name.to_string() },
                )
            ]
        };

        assert_eq!(serde_json::to_string(&list_products(&connection).unwrap()).unwrap(), 
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
            ]).unwrap());

        Ok(())
    });
}

#[test]
fn show_product_test() {
    use diesel::result::Error;
    use super::connect::establish_connection_test;
    use super::models::{ NewCompleteProduct, NewVariant, NewVariantValue, NewProduct, Product };
    let connection = establish_connection_test();
    connection.test_transaction::<_, Error, _>(|| {
        let product_id =
            create_product(NewCompleteProduct {
                product: NewProduct { name: "boots".to_string(), cost: 13.23, active: true },
                variants: vec![
                    NewVariantValue {
                        variant: NewVariant { name: "size".to_string() },
                        values: vec![
                            Some(12.to_string()),
                            Some(14.to_string()),
                            Some(16.to_string()),
                            Some(18.to_string())
                        ]
                    }
                ]
            }, &connection).unwrap();

        assert_eq!(
            serde_json::to_string(&show_product(product_id, &connection).unwrap()).unwrap(),
            serde_json::to_string(
                &(
                    Product { id: 1, name: "boots".to_string(), cost: 13.23, active: true },
                    vec![
                        (
                            ProductVariant {
                                id: 1,
                                variant_id: 1,
                                product_id: 1,
                                value: Some("12".to_string()),
                            },
                            Variant { id: 1, name: "size".to_string() }
                        ),
                        (
                            ProductVariant {
                                id: 2,
                                variant_id: 1,
                                product_id: 1,
                                value: Some("14".to_string()),
                            },
                            Variant { id: 1, name: "size".to_string() }
                        ),
                        (
                            ProductVariant {
                                id: 3,
                                variant_id: 1,
                                product_id: 1,
                                value: Some("16".to_string()),
                            },
                            Variant { id: 1, name: "size".to_string()}
                        ),
                        (
                            ProductVariant {
                                id: 4,
                                variant_id: 1,
                                product_id: 1,
                                value: Some("18".to_string()),
                            },
                            Variant { id: 1, name: "size".to_string() }
                        )
                    ]
                )
            ).unwrap()
        );


        Ok(())
    });
}

#[test]
fn search_products_test() {
    use diesel::result::Error;
    use super::connect::establish_connection_test;
    use super::models::{ NewCompleteProduct, NewVariant, NewVariantValue, NewProduct, Product, Variant, ProductVariant };
    let connection = establish_connection_test();
    connection.test_transaction::<_, Error, _>(|| {
        let variants = vec![
            NewVariantValue {
                variant: NewVariant { name: "size".to_string() },
                values: vec![ Some(12.to_string()) ]
            }
        ];

        create_product(NewCompleteProduct {
            product: NewProduct { name: "boots".to_string(), cost: 13.23, active: true },
            variants: variants.clone()
        }, &connection).unwrap();
        create_product(NewCompleteProduct {
            product: NewProduct { name: "high heels".to_string(), cost: 20.99, active: true },
            variants: variants.clone()
        }, &connection).unwrap();
        create_product(NewCompleteProduct {
            product: NewProduct { name: "running shoes".to_string(), cost: 10.99, active: true },
            variants: variants.clone()
        }, &connection).unwrap();

        assert_eq!(
            serde_json::to_string(&search_products("shoes".to_string(), &connection).unwrap()).unwrap(),
            serde_json::to_string(&vec![
                (
                    Product {
                        id: 3,
                        name: "running shoes".to_string(),
                        cost: 10.99,
                        active: true
                    },
                    vec![
                        (
                            ProductVariant {
                                id: 3,
                                variant_id: 1,
                                product_id: 3,
                                value: Some( "12".to_string() ),
                            },
                            Variant {
                                id: 1,
                                name: "size".to_string(),
                            }
                        )
                    ]
                )
            ]).unwrap()
        );

        Ok(())
    });
}
