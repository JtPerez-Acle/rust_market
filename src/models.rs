// Copyright 2024 [Your Name or Company Name]
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use diesel::{Queryable, Selectable, Identifiable, Associations, Insertable};
use chrono::{NaiveDateTime, NaiveDate};
use serde::{Serialize, Deserialize};
use bigdecimal::BigDecimal;

// Import schema modules
use crate::schema::{users, equipment, equipment_categories, orders, order_items, reviews, maintenance_records, equipment_images};

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub company_name: Option<String>,
    pub business_type: Option<String>,
    pub contact_number: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub company_name: Option<String>,
    pub business_type: Option<String>,
    pub contact_number: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl NewUser {
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            username,
            email,
            password_hash,
            company_name: None,
            business_type: None,
            contact_number: None,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = equipment_categories)]
pub struct EquipmentCategory {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub parent_category_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = equipment_categories)]
pub struct NewEquipmentCategory {
    pub name: String,
    pub description: Option<String>,
    pub parent_category_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(EquipmentCategory, foreign_key = category_id))]
#[diesel(table_name = equipment)]
pub struct Equipment {
    pub id: i32,
    pub category_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub manufacturer: String,
    pub model_number: String,
    pub year_manufactured: Option<i32>,
    pub condition: String,
    pub price: BigDecimal,
    pub stock_level: i32,
    pub specifications: Option<serde_json::Value>,
    pub weight_kg: Option<BigDecimal>,
    pub dimensions_cm: Option<String>,
    pub power_requirements: Option<String>,
    pub certification_info: Option<String>,
    pub warranty_info: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = equipment)]
pub struct NewEquipment {
    pub category_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub manufacturer: String,
    pub model_number: String,
    pub year_manufactured: Option<i32>,
    pub condition: String,
    pub price: BigDecimal,
    pub stock_level: i32,
    pub specifications: Option<serde_json::Value>,
    pub weight_kg: Option<BigDecimal>,
    pub dimensions_cm: Option<String>,
    pub power_requirements: Option<String>,
    pub certification_info: Option<String>,
    pub warranty_info: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(Equipment))]
#[diesel(table_name = equipment_images)]
pub struct EquipmentImage {
    pub id: i32,
    pub equipment_id: i32,
    pub image_url: String,
    pub is_primary: Option<bool>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = equipment_images)]
pub struct NewEquipmentImage {
    pub equipment_id: i32,
    pub image_url: String,
    pub is_primary: Option<bool>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(User))]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub status: String,
    pub total_amount: BigDecimal,
    pub shipping_address: String,
    pub shipping_method: String,
    pub tracking_number: Option<String>,
    pub estimated_delivery_date: Option<NaiveDate>,
    pub special_instructions: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = orders)]
pub struct NewOrder {
    pub user_id: i32,
    pub status: String,
    pub total_amount: BigDecimal,
    pub shipping_address: String,
    pub shipping_method: String,
    pub tracking_number: Option<String>,
    pub estimated_delivery_date: Option<NaiveDate>,
    pub special_instructions: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(Order))]
#[diesel(belongs_to(Equipment))]
#[diesel(table_name = order_items)]
pub struct OrderItem {
    pub id: i32,
    pub order_id: i32,
    pub equipment_id: i32,
    pub quantity: i32,
    pub price_at_time: BigDecimal,
    pub warranty_selected: Option<bool>,
    pub special_requirements: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = order_items)]
pub struct NewOrderItem {
    pub order_id: i32,
    pub equipment_id: i32,
    pub quantity: i32,
    pub price_at_time: BigDecimal,
    pub warranty_selected: Option<bool>,
    pub special_requirements: Option<String>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(Equipment))]
#[diesel(belongs_to(User))]
#[diesel(table_name = reviews)]
pub struct Review {
    pub id: i32,
    pub equipment_id: i32,
    pub user_id: i32,
    pub rating: i32,
    pub review_text: Option<String>,
    pub usage_duration: Option<String>,
    pub pros: Option<String>,
    pub cons: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = reviews)]
pub struct NewReview {
    pub equipment_id: i32,
    pub user_id: i32,
    pub rating: i32,
    pub review_text: Option<String>,
    pub usage_duration: Option<String>,
    pub pros: Option<String>,
    pub cons: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(Equipment))]
#[diesel(table_name = maintenance_records)]
pub struct MaintenanceRecord {
    pub id: i32,
    pub equipment_id: i32,
    pub service_date: NaiveDate,
    pub service_type: String,
    pub description: Option<String>,
    pub performed_by: Option<String>,
    pub next_service_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = maintenance_records)]
pub struct NewMaintenanceRecord {
    pub equipment_id: i32,
    pub service_date: NaiveDate,
    pub service_type: String,
    pub description: Option<String>,
    pub performed_by: Option<String>,
    pub next_service_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
}
