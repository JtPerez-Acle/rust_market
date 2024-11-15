// Copyright 2024 Jose Tomas Perez-Acle
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

use diesel::prelude::*;
use chrono::{DateTime, Utc};
use bigdecimal::BigDecimal;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Asset {
    pub id: i32,
    pub name: String,
    pub price: BigDecimal,
    pub stock: i32,
    pub image_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::assets)]
pub struct NewAsset {
    pub name: String,
    pub price: BigDecimal,
    pub stock: i32,
    pub image_url: String,
}

impl NewAsset {
    pub fn new(name: String, price: BigDecimal, stock: i32, image_url: String) -> Self {
        NewAsset {
            name,
            price,
            stock,
            image_url,
        }
    }
}
