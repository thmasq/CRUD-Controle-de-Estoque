// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    products (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 50]
        sku -> Varchar,
        category_id -> Nullable<Uuid>,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    stock_items (id) {
        id -> Uuid,
        product_id -> Uuid,
        warehouse_id -> Uuid,
        quantity -> Int4,
        unit_cost -> Numeric,
        last_restocked -> Timestamptz,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        is_active -> Bool,
    }
}

diesel::table! {
    stock_transactions (id) {
        id -> Uuid,
        stock_item_id -> Uuid,
        quantity -> Int4,
        #[max_length = 20]
        transaction_type -> Varchar,
        #[max_length = 100]
        reference_number -> Nullable<Varchar>,
        notes -> Nullable<Text>,
        created_at -> Timestamptz,
        #[max_length = 100]
        created_by -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 20]
        role -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    warehouses (id) {
        id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        location -> Text,
        contact_info -> Nullable<Text>,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(products -> categories (category_id));
diesel::joinable!(stock_items -> products (product_id));
diesel::joinable!(stock_items -> warehouses (warehouse_id));
diesel::joinable!(stock_transactions -> stock_items (stock_item_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    products,
    stock_items,
    stock_transactions,
    users,
    warehouses,
);
