// @generated automatically by Diesel CLI.

diesel::table! {
    order_items (id) {
        id -> Int4,
        order_id -> Int4,
        product_id -> Int4,
        quantity -> Int4,
        price_at_time -> Numeric,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        user_id -> Int4,
        status -> Varchar,
        total_amount -> Numeric,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        price -> Numeric,
        stock_level -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(order_items -> orders (order_id));
diesel::joinable!(order_items -> products (product_id));
diesel::joinable!(orders -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    order_items,
    orders,
    products,
    users,
);
