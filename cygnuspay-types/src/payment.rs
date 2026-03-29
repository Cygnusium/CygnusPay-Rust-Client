// Core payment structs

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Debug)]
pub struct PaymentRequest {
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl PaymentRequest {
    pub fn new(
        amount: f64,
    ) -> PaymentRequest {
        PaymentRequest {
            amount,
            currency: None,
            description: None,
            metadata: None,
            title: None,
        }
    }
    
    pub fn set_currency(&mut self, currency: Option<String>) {
        self.currency = currency
    }
    
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description
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
    #[serde(deserialize_with = "deserialize_to_datetime")]
    pub timestamp: DateTime<Utc>,
    pub tx_hash: String,
}

#[derive(Deserialize, Debug)]
pub struct PaymentResponse {
    pub confirmed_amount: Option<f64>,
    pub currency: Option<String>,
    pub deposits: Option<Vec<Deposit>>,
    pub id: Option<String>,
    #[serde(rename = "is_confirmed")]
    pub confirmed: Option<bool>,
    #[serde(rename = "message")]
    pub error_message: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub status: Option<String>,
    pub success: bool,
}

fn deserialize_to_datetime<'de, D: serde::Deserializer<'de>>(d: D) -> Result<DateTime<Utc>, D::Error> {
    let s: String = serde::Deserialize::deserialize(d)?;
    s.parse::<DateTime<Utc>>().map_err(serde::de::Error::custom)
}

