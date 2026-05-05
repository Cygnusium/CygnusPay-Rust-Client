use crate::shared::BaseResponse;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PaymentDeleteResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
}
