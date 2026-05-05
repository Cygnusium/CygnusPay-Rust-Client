// Builders for constructing structs

use cygnuspay_types::payment_request::{ExpiryUnit, PaymentRequest, PaymentType};

pub struct PaymentBuilder {
    amount: f64,
    currency: String,
    description: Option<String>,
    expiry_unit: Option<ExpiryUnit>,
    expiry_value: Option<u64>,
    metadata: Option<serde_json::Value>,
    payment_type: Option<PaymentType>,
    title: Option<String>,
}

impl PaymentBuilder {
    pub fn new(amount: impl Into<f64>, currency: impl Into<String>) -> Self {
        Self {
            amount: amount.into(),
            currency: currency.into(),
            description: None,
            expiry_unit: None,
            expiry_value: None,
            metadata: None,
            payment_type: None,
            title: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn expiry_unit(mut self, expiry_unit: ExpiryUnit) -> Self {
        self.expiry_unit = Some(expiry_unit);
        self
    }

    pub fn expiry_value(mut self, expiry_value: impl Into<u64>) -> Self {
        self.expiry_value = Some(expiry_value.into());
        self
    }

    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn payment_type(mut self, payment_type: PaymentType) -> Self {
        self.payment_type = Some(payment_type);
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn build(self) -> PaymentRequest {
        // we should do some validation here
        // for example, if expiry unit is provided
        // expiry_value should be provided too

        PaymentRequest {
            amount: self.amount,
            currency: self.currency,
            description: self.description,
            expiry_unit: self.expiry_unit,
            expiry_value: self.expiry_value,
            metadata: self.metadata,
            payment_type: self.payment_type,
            title: self.title,
        }
    }
}

fn main() {
    let test = PaymentBuilder::new(10.3, "USDT").title("Test Transaction");
}
