use serde::{ Serialize, Deserialize };
use super::schema::products;
use super::schema::variants;
use super::schema::products_variants;

#[derive(Identifiable, Queryable, PartialEq, Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub cost: f64,
    pub active: bool,
}

#[derive(Insertable, Debug)]
#[table_name="products"]
pub struct NewProduct {
    pub name: String,
    pub cost: f64,
    pub active: bool,
}

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize)]
#[table_name = "variants"]
pub struct Variant {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Debug, Clone)]
#[table_name="variants"]
pub struct NewVariant {
    pub name: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[belongs_to(Product)]
#[table_name="products_variants"]
pub struct ProductVariant {
    pub id: i32,
    pub product_id: i32,
    pub variant_id: i32,
    pub value: Option<String>
}

#[derive(Insertable, Debug)]
#[table_name="products_variants"]
pub struct NewProductVariant {
    pub product_id: i32,
    pub variant_id: i32,
    pub value: Option<String>
}



#[derive(Clone, Debug)]
pub struct NewVariantValue {
    pub variant: NewVariant,
    pub values: Vec<Option<String>>
}

pub struct NewCompleteProduct {
    pub product: NewProduct,
    pub variants: Vec<NewVariantValue>
}