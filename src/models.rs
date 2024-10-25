use diesel::prelude::*;
use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use bigdecimal::BigDecimal;

// Import schema modules
use crate::schema::{users, products, orders, order_items};

#[derive(Queryable, Identifiable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl NewUser {
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            username,
            email,
            password_hash,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = products)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub stock_level: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub stock_level: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl NewProduct {
    pub fn new(name: String, description: Option<String>, price: BigDecimal, stock_level: i32) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            name,
            description,
            price,
            stock_level,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(User))]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub status: String,
    pub total_amount: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = orders)]
pub struct NewOrder {
    pub user_id: i32,
    pub status: String,
    pub total_amount: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl NewOrder {
    pub fn new(user_id: i32, status: String, total_amount: BigDecimal) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            user_id,
            status,
            total_amount,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(Order))]
#[diesel(belongs_to(Product))]
#[diesel(table_name = order_items)]
pub struct OrderItem {
    pub id: i32,
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price_at_time: BigDecimal,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = order_items)]
pub struct NewOrderItem {
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub price_at_time: BigDecimal,
}
