// Core payment structs

use crate::base::BaseResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ExpiryUnit {
    MINUTES,
    HOURS,
    DAYS,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    ACTIVE,
    INACTIVE,
}

#[derive(Serialize, Debug)]
pub struct PaymentRequest {
    pub amount: f64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_unit: Option<ExpiryUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_value: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl PaymentRequest {
    pub fn new(amount: f64, currency: String) -> PaymentRequest {
        PaymentRequest {
            amount,
            currency,
            description: None,
            expiry_unit: None,
            expiry_value: None,
            metadata: None,
            title: None,
        }
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description
    }

    pub fn set_expiry_value(&mut self, expiry_value: Option<u64>) {
        self.expiry_value = expiry_value;
    }

    pub fn set_expiry_unit(&mut self, expiry_unit: Option<ExpiryUnit>) {
        self.expiry_unit = expiry_unit;
    }

    pub fn set_metadata(&mut self, metadata: Option<serde_json::Value>) {
        self.metadata = metadata
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title
    }
}

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

#[derive(Deserialize, Debug)]
pub struct PaymentResponseListItem {
    pub created_at: DateTime<Utc>,
    pub currency: String,
    pub expires_at: DateTime<Utc>,
    pub id: String,
    pub is_expired: bool,
    #[serde(rename = "type")]
    pub payment_type: String,
    pub status: String,
    pub title: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct PaymentResponseList {
    pub base: BaseResponse,
    pub count: Option<u64>,
    #[serde(rename = "payments")]
    pub items: Option<Vec<PaymentResponseListItem>>,
}
