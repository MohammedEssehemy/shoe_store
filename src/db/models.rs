use super::schema::products;
use super::schema::products_variants;
use super::schema::variants;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, PartialEq, Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub cost: f64,
    pub active: bool,
}

#[derive(Insertable, Debug, AsChangeset, Serialize, Deserialize, Clone)]
#[table_name = "products"]
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

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "variants"]
pub struct NewVariant {
    pub name: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Serialize, Deserialize)]
#[belongs_to(Product)]
#[table_name = "products_variants"]
pub struct ProductVariant {
    pub id: i32,
    pub product_id: i32,
    pub variant_id: i32,
    pub value: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "products_variants"]
pub struct NewProductVariant {
    pub product_id: i32,
    pub variant_id: i32,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewVariantValue {
    pub variant: NewVariant,
    pub values: Vec<Option<String>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NewCompleteProduct {
    pub product: NewProduct,
    pub variants: Vec<NewVariantValue>,
}


#[derive(Insertable, Queryable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[table_name="variants"]
pub struct FormVariant {
    pub id: Option<i32>,
    pub name: String
}

#[derive(Insertable, Debug, AsChangeset, Serialize, Deserialize)]
#[table_name="products_variants"]
pub struct FormProductVariant {
    pub id: Option<i32>,
    pub variant_id: Option<i32>,
    pub product_id: i32,
    pub value: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct FormProductVariantComplete {
    pub variant: Option<FormVariant>,
    pub product_variant: FormProductVariant,
}

#[derive(Serialize, Deserialize)]
pub struct FormProduct {
    pub product: NewProduct,
    pub variants: Vec<FormProductVariantComplete>
}