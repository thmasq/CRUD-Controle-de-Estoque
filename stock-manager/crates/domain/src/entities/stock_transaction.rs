use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    In,
    Out,
    Adjustment,
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::In => write!(f, "IN"),
            TransactionType::Out => write!(f, "OUT"),
            TransactionType::Adjustment => write!(f, "ADJUSTMENT"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockTransaction {
    pub id: Uuid,
    pub stock_item_id: Uuid,
    pub quantity: i32,
    pub transaction_type: TransactionType,
    pub reference_number: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
}
