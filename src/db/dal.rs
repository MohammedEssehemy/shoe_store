use super::models::{NewCompleteProduct, Product, ProductVariant, Variant, FormProduct};
use super::schema::{products, products_variants, variants};
use anyhow::Result;
use diesel::{
    sqlite::SqliteConnection, BelongingToDsl, Connection, ExpressionMethods, GroupedBy, QueryDsl,
    RunQueryDsl, TextExpressionMethods,
};

no_arg_sql_function!(last_insert_rowid, diesel::sql_types::Integer);

pub fn create_product(new_product: NewCompleteProduct, conn: &SqliteConnection) -> Result<i32> {
    conn.transaction(|| {
        diesel::insert_into(products::table)
            .values(new_product.product)
            .execute(conn)?;

        let last_product_id = diesel::select(last_insert_rowid).first(conn)?;

        for new_variant in new_product.variants {
            let variants_result = variants::table
                .filter(variants::name.eq(&new_variant.variant.name))
                .limit(1)
                .load::<Variant>(conn)?;

            let last_variant_id = match variants_result.first() {
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
                    .values((
                        products_variants::product_id.eq(last_product_id),
                        products_variants::variant_id.eq(last_variant_id),
                        products_variants::value.eq(new_value),
                    ))
                    .execute(conn)?;
            }
        }
        Ok(last_product_id)
    })
}

pub fn show_product(id: i32, conn: &SqliteConnection) -> Result<(Product, Vec<(ProductVariant, Variant)>)> {
    let product_result = products::table.find(id).get_result::<Product>(conn)?;

    let variants_result = ProductVariant::belonging_to(&product_result)
        .inner_join(variants::table)
        .load::<(ProductVariant, Variant)>(conn)?;

    Ok((product_result, variants_result))
}

pub fn list_products(conn: &SqliteConnection) -> Result<Vec<(Product, Vec<(ProductVariant, Variant)>)>> {
    let products_result = products::table.load::<Product>(conn)?;
    let variants_result = ProductVariant::belonging_to(&products_result)
        .inner_join(variants::table)
        .load::<(ProductVariant, Variant)>(conn)?
        .grouped_by(&products_result);
    let data = products_result
        .into_iter()
        .zip(variants_result)
        .collect::<Vec<_>>();

    Ok(data)
}

pub fn search_products(search: String, conn: &SqliteConnection) -> Result<Vec<(Product, Vec<(ProductVariant, Variant)>)>> {
    let pattern = format!("%{}%", search);
    let products_result = products::table
        .filter(products::name.like(pattern))
        .load::<Product>(conn)?;
    let variants_result = ProductVariant::belonging_to(&products_result)
        .inner_join(variants::table)
        .load::<(ProductVariant, Variant)>(conn)?
        .grouped_by(&products_result);
    let data = products_result
        .into_iter()
        .zip(variants_result)
        .collect::<Vec<_>>();

    Ok(data)
}

pub fn update_product(product_id: i32, form_product: FormProduct, conn: &SqliteConnection) -> Result<i32> {
    conn.transaction(|| {
        diesel::update(products::table.find(product_id))
            .set(&form_product.product)
            .execute(conn)?;

        for mut form_product_variant in form_product.variants {
            if form_product_variant.product_variant.variant_id.is_none() {
                diesel::insert_into(variants::dsl::variants)
                    .values(form_product_variant.variant)
                    .execute(conn)?;
                let last_variant_id: i32 = diesel::select(last_insert_rowid).first(conn)?;
                form_product_variant.product_variant.variant_id = Some(last_variant_id);            
            }

            if let Some(product_variant_id) = form_product_variant.product_variant.id {
                diesel::update(products_variants::table.find(product_variant_id))
                    .set(&form_product_variant.product_variant)
                    .execute(conn)?;
            } else {
                diesel::insert_into(products_variants::table)
                    .values(&form_product_variant.product_variant)
                    .execute(conn)?;
            }
        }

        Ok(product_id)
    })
}

pub fn delete_product(id: i32, conn: &SqliteConnection) ->  Result<i32> {
    diesel::delete(products::table.find(id))
        .execute(conn)?;

    Ok(id)
}