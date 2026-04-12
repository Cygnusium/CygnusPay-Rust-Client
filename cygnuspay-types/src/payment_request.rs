// endpoint: POST /v1/payments

use crate::shared::BaseResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ExpiryUnit {
    MINUTES,
    HOURS,
    DAYS,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PaymentType {
    ONETIME,
    DONATION,
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub payment_type: Option<PaymentType>,
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
            payment_type: None,
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

    pub fn set_payment_type(&mut self, payment_type: Option<PaymentType>) {
        self.payment_type = payment_type
    }
}

#[derive(Deserialize, Debug)]
pub struct PaymentRequestResponse {
    pub amount: Option<f64>,
    #[serde(flatten)]
    pub base: BaseResponse,
    pub currency: Option<String>,
    pub payment_id: Option<String>,
    pub payment_url: Option<String>,
}
