use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentRequest {
    pub amount: f64,
    pub currency: Option<String>,
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
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
    
    pub fn set_currency(&mut self, currency: String) {
        self.currency = Some(currency);
    }
    
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }
    
    pub fn set_metadata(&mut self, metadata: serde_json::Value) {
        self.metadata = Some(metadata);
    }
    
    pub fn set_title(&mut self, title: String) {
        self.title = Some(title);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Deposit {
    pub amount: f64,
    pub timestamp: String,
    pub tx_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
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

