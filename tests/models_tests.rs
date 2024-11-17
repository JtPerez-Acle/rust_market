use diesel::prelude::*;
use bigdecimal::BigDecimal;
use std::str::FromStr;
use uuid::Uuid;
use rust_market::{
    models::*,
    schema::*,
    db::establish_connection_pool,
    test_helpers::{setup, cleanup_database},
};

// Helper Functions
fn generate_unique_identifier() -> String {
    Uuid::new_v4().to_string().split('-').next().unwrap().to_string()
}

#[test]
fn test_create_new_user() {
    setup();
    let pool = establish_connection_pool(None)
        .expect("Failed to create connection pool");
    let conn = &mut pool.get()
        .expect("Failed to get db connection");

    cleanup_database(&pool);

    let unique_id = generate_unique_identifier();
    let new_user = NewUser {
        username: format!("Test User {}", unique_id),
        email: format!("testuser{}@example.com", unique_id),
        password_hash: "hashed_password".to_string(),
        company_name: Some("Mining Corp".to_string()),
        business_type: Some("buyer".to_string()),
        contact_number: Some("+1234567890".to_string()),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(conn);

    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.username, new_user.username);
    assert_eq!(user.email, new_user.email);
    assert_eq!(user.password_hash, new_user.password_hash);
    assert_eq!(user.company_name, new_user.company_name);
    assert_eq!(user.business_type, new_user.business_type);
    assert_eq!(user.contact_number, new_user.contact_number);

    cleanup_database(&pool);
}

#[test]
fn test_create_equipment_category() {
    setup();
    let pool = establish_connection_pool(None)
        .expect("Failed to create connection pool");
    let conn = &mut pool.get()
        .expect("Failed to get db connection");

    cleanup_database(&pool);

    let unique_id = generate_unique_identifier();
    let new_category = NewEquipmentCategory {
        name: format!("Mining Vehicles {}", unique_id),
        description: Some("Heavy-duty mining vehicles and equipment".to_string()),
        parent_category_id: None,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let result = diesel::insert_into(equipment_categories::table)
        .values(&new_category)
        .get_result::<EquipmentCategory>(conn);

    assert!(result.is_ok());
    let category = result.unwrap();
    assert_eq!(category.name, new_category.name);
    assert_eq!(category.description, new_category.description);

    cleanup_database(&pool);
}

#[test]
fn test_create_equipment() {
    setup();
    let pool = establish_connection_pool(None)
        .expect("Failed to create connection pool");
    let conn = &mut pool.get()
        .expect("Failed to get db connection");

    cleanup_database(&pool);

    // First create a category
    let new_category = NewEquipmentCategory {
        name: "Mining Vehicles".to_string(),
        description: Some("Heavy-duty mining vehicles and equipment".to_string()),
        parent_category_id: None,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let category = diesel::insert_into(equipment_categories::table)
        .values(&new_category)
        .get_result::<EquipmentCategory>(conn)
        .expect("Failed to create category");

    // Then create equipment
    let unique_id = generate_unique_identifier();
    let new_equipment = NewEquipment {
        category_id: category.id,
        name: format!("Mining Truck {}", unique_id),
        description: Some("Heavy-duty mining truck for ore transportation".to_string()),
        manufacturer: "CAT".to_string(),
        model_number: format!("MT-{}", unique_id),
        year_manufactured: Some(2023),
        condition: "new".to_string(),
        price: BigDecimal::from_str("250000.00").unwrap(),
        stock_level: 5,
        specifications: Some(serde_json::json!({
            "capacity": "100 tons",
            "engine": "diesel",
            "power": "2000 hp"
        })),
        weight_kg: Some(BigDecimal::from_str("45000.00").unwrap()),
        dimensions_cm: Some("800x300x400".to_string()),
        power_requirements: Some("diesel".to_string()),
        certification_info: Some("ISO 9001:2015".to_string()),
        warranty_info: Some("3 years full warranty".to_string()),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let result = diesel::insert_into(equipment::table)
        .values(&new_equipment)
        .get_result::<Equipment>(conn);

    if let Err(ref e) = result {
        println!("Error creating equipment: {:?}", e);
    }
    assert!(result.is_ok());
    let equipment = result.unwrap();
    assert_eq!(equipment.name, new_equipment.name);
    assert_eq!(equipment.manufacturer, new_equipment.manufacturer);
    assert_eq!(equipment.price, new_equipment.price);
    assert_eq!(equipment.stock_level, new_equipment.stock_level);

    cleanup_database(&pool);
}

#[test]
fn test_create_order() {
    setup();
    let pool = establish_connection_pool(None)
        .expect("Failed to create connection pool");
    let conn = &mut pool.get()
        .expect("Failed to get db connection");

    cleanup_database(&pool);

    conn.transaction(|conn| {
        // Create test user
        let new_user = NewUser {
            username: "Test User".to_string(),
            email: "testuser@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            company_name: Some("Mining Corp".to_string()),
            business_type: Some("buyer".to_string()),
            contact_number: Some("+1234567890".to_string()),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        let user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(conn)
            .expect("Failed to create user");

        // Create equipment category with a unique name
        let unique_id = generate_unique_identifier();
        let new_category = NewEquipmentCategory {
            name: format!("Mining Vehicles {}", unique_id),
            description: Some("Heavy-duty mining vehicles and equipment".to_string()),
            parent_category_id: None,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        let category = diesel::insert_into(equipment_categories::table)
            .values(&new_category)
            .get_result::<EquipmentCategory>(conn)
            .expect("Failed to create category");

        // Create equipment with unique name and model number
        let new_equipment = NewEquipment {
            category_id: category.id,
            name: format!("Mining Truck {}", unique_id),
            description: Some("Heavy-duty mining truck".to_string()),
            manufacturer: "CAT".to_string(),
            model_number: format!("MT-{}", unique_id),
            year_manufactured: Some(2023),
            condition: "new".to_string(),
            price: BigDecimal::from_str("250000.00").unwrap(),
            stock_level: 5,
            specifications: Some(serde_json::json!({
                "capacity": "100 tons",
                "engine": "diesel"
            })),
            weight_kg: Some(BigDecimal::from_str("45000.00").unwrap()),
            dimensions_cm: Some("800x300x400".to_string()),
            power_requirements: Some("diesel".to_string()),
            certification_info: Some("ISO 9001:2015".to_string()),
            warranty_info: Some("3 years full warranty".to_string()),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        let equipment = diesel::insert_into(equipment::table)
            .values(&new_equipment)
            .get_result::<Equipment>(conn)
            .expect("Failed to create equipment");

        // Create order
        let new_order = NewOrder {
            user_id: user.id,
            status: "pending".to_string(),
            total_amount: BigDecimal::from_str("250000.00").unwrap(),
            shipping_address: "123 Mining Ave, Rock City".to_string(),
            shipping_method: "freight".to_string(),
            tracking_number: None,
            estimated_delivery_date: Some(chrono::Utc::now().naive_utc().date()),
            special_instructions: Some("Handle with care".to_string()),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        let order = diesel::insert_into(orders::table)
            .values(&new_order)
            .get_result::<Order>(conn)
            .expect("Failed to create order");

        // Create order item
        let new_order_item = NewOrderItem {
            order_id: order.id,
            equipment_id: equipment.id,
            quantity: 1,
            price_at_time: equipment.price.clone(),
            warranty_selected: Some(true),
            special_requirements: Some("Require installation".to_string()),
        };

        let order_item = diesel::insert_into(order_items::table)
            .values(&new_order_item)
            .get_result::<OrderItem>(conn)
            .expect("Failed to create order item");

        assert_eq!(order_item.order_id, order.id);
        assert_eq!(order_item.equipment_id, equipment.id);
        assert_eq!(order_item.quantity, 1);
        assert_eq!(order_item.price_at_time, equipment.price);

        Ok::<_, diesel::result::Error>(())
    }).expect("Transaction failed");

    cleanup_database(&pool);
}
