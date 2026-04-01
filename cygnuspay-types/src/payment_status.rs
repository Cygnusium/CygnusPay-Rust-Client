// Core payment structs

use crate::shared::{BaseResponse, Status};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Deposit {
    pub amount: f64,
    pub timestamp: DateTime<Utc>,
    pub tx_hash: String,
}

#[derive(Deserialize, Debug)]
pub struct PaymentStatusResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub confirmed_amount: Option<f64>,
    pub currency: Option<String>,
    pub deposits: Option<Vec<Deposit>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub id: Option<String>,
    pub is_expired: Option<bool>,
    pub is_confirmed: Option<bool>,
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "type")]
    pub payment_type: Option<String>,
    pub status: Option<Status>,
}
