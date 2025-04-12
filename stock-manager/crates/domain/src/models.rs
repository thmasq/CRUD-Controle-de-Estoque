use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub sku: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Product {
    pub fn new(name: String, description: Option<String>, sku: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            sku,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, name: String, description: Option<String>, sku: String) {
        self.name = name;
        self.description = description;
        self.sku = sku;
        self.updated_at = Utc::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockItem {
    pub id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub location: String,
    pub unit_cost: BigDecimal,
    pub last_restocked: DateTime<Utc>,
}

impl StockItem {
    pub fn new(product_id: Uuid, quantity: i32, location: String, unit_cost: BigDecimal) -> Self {
        Self {
            id: Uuid::new_v4(),
            product_id,
            quantity,
            location,
            unit_cost,
            last_restocked: Utc::now(),
        }
    }

    pub fn restock(&mut self, additional_quantity: i32) {
        self.quantity += additional_quantity;
        self.last_restocked = Utc::now();
    }

    pub fn update_quantity(&mut self, new_quantity: i32) -> Result<(), &'static str> {
        if new_quantity < 0 {
            return Err("Quantity cannot be negative");
        }
        self.quantity = new_quantity;
        self.last_restocked = Utc::now();
        Ok(())
    }
}
