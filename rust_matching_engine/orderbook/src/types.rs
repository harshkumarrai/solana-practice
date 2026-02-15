use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrder {
    pub price: u64,
    pub quantity: u64,
    pub user_id: String,
    pub side: Side,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteOrder {
    pub order_id: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionReport {
    pub filled_quantity: u64,
    pub remaining_quantity: u64,
    pub average_price: u64,
}
