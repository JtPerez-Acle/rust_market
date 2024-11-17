// @generated automatically by Diesel CLI.

diesel::table! {
    equipment (id) {
        id -> Int4,
        category_id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        manufacturer -> Varchar,
        model_number -> Varchar,
        year_manufactured -> Nullable<Int4>,
        condition -> Varchar,
        price -> Numeric,
        stock_level -> Int4,
        specifications -> Nullable<Jsonb>,
        weight_kg -> Nullable<Numeric>,
        dimensions_cm -> Nullable<Varchar>,
        power_requirements -> Nullable<Varchar>,
        certification_info -> Nullable<Text>,
        warranty_info -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    equipment_categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        parent_category_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    equipment_images (id) {
        id -> Int4,
        equipment_id -> Int4,
        image_url -> Varchar,
        is_primary -> Nullable<Bool>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    maintenance_records (id) {
        id -> Int4,
        equipment_id -> Int4,
        service_date -> Date,
        service_type -> Varchar,
        description -> Nullable<Text>,
        performed_by -> Nullable<Varchar>,
        next_service_date -> Nullable<Date>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    order_items (id) {
        id -> Int4,
        order_id -> Int4,
        equipment_id -> Int4,
        quantity -> Int4,
        price_at_time -> Numeric,
        warranty_selected -> Nullable<Bool>,
        special_requirements -> Nullable<Text>,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        user_id -> Int4,
        status -> Varchar,
        total_amount -> Numeric,
        shipping_address -> Text,
        shipping_method -> Varchar,
        tracking_number -> Nullable<Varchar>,
        estimated_delivery_date -> Nullable<Date>,
        special_instructions -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    reviews (id) {
        id -> Int4,
        equipment_id -> Int4,
        user_id -> Int4,
        rating -> Int4,
        review_text -> Nullable<Text>,
        usage_duration -> Nullable<Varchar>,
        pros -> Nullable<Text>,
        cons -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    technical_documents (id) {
        id -> Int4,
        equipment_id -> Int4,
        document_type -> Varchar,
        document_url -> Varchar,
        version -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        company_name -> Nullable<Varchar>,
        business_type -> Nullable<Varchar>,
        contact_number -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(equipment -> equipment_categories (category_id));
diesel::joinable!(equipment_images -> equipment (equipment_id));
diesel::joinable!(maintenance_records -> equipment (equipment_id));
diesel::joinable!(order_items -> equipment (equipment_id));
diesel::joinable!(order_items -> orders (order_id));
diesel::joinable!(orders -> users (user_id));
diesel::joinable!(reviews -> equipment (equipment_id));
diesel::joinable!(reviews -> users (user_id));
diesel::joinable!(technical_documents -> equipment (equipment_id));

diesel::allow_tables_to_appear_in_same_query!(
    equipment,
    equipment_categories,
    equipment_images,
    maintenance_records,
    order_items,
    orders,
    reviews,
    technical_documents,
    users,
);
