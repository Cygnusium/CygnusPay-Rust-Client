// endpoint: POST /v1/donations

use crate::shared::BaseResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct DonationRequestResponse {
    pub amount: Option<f64>,
    #[serde(flatten)]
    pub base: BaseResponse,
    pub currency: Option<String>,
    pub payment_id: Option<String>,
    pub payment_url: Option<String>,
}
