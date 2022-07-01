table! {
    products (id) {
        id -> Integer,
        name -> Text,
        cost -> Double,
        active -> Bool,
    }
}

table! {
    products_variants (id) {
        id -> Integer,
        product_id -> Integer,
        variant_id -> Integer,
        value -> Nullable<Text>,
    }
}

table! {
    variants (id) {
        id -> Integer,
        name -> Text,
    }
}

joinable!(products_variants -> products (product_id));
joinable!(products_variants -> variants (variant_id));

allow_tables_to_appear_in_same_query!(
    products,
    products_variants,
    variants,
);
