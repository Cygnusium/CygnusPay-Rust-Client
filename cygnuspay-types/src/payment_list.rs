// endpoint: GET /v1/payments

use crate::shared::{BaseResponse, Status};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PaymentListItem {
    pub created_at: DateTime<Utc>,
    pub currency: String,
    pub expires_at: DateTime<Utc>,
    pub id: String,
    pub is_expired: bool,
    #[serde(rename = "type")]
    pub payment_type: String,
    pub status: Status,
    pub title: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct PaymentListResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    pub count: Option<u64>,
    #[serde(rename = "payments")]
    pub items: Option<Vec<PaymentListItem>>,
}
