use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: Option<String>,
    pub sku: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProductRequest {
    pub name: String,
    pub description: Option<String>,
    pub sku: String,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub sku: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStockItemRequest {
    pub product_id: Uuid,
    pub quantity: i32,
    pub location: String,
    pub unit_cost: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStockQuantityRequest {
    pub quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct StockItemResponse {
    pub id: String,
    pub product_id: String,
    pub quantity: i32,
    pub location: String,
    pub unit_cost: String,
    pub last_restocked: String,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub message: String,
}
