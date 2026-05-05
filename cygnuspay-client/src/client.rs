use cygnuspay_types::payment_list::PaymentListResponse;
use cygnuspay_types::payment_status::PaymentStatusResponse;

pub struct CygnusClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

pub enum CygnusError {
    Http(reqwest::Error),
    Api(String),
    Deserialize(serde_json::Error),
}

impl CygnusClient {
    pub fn new(api_key: String, base_url: String, client: reqwest::Client) -> Self {
        CygnusClient {
            api_key,
            base_url,
            client,
        }
    }

    pub async fn get_payment(
        &self,
        payment_id: String,
    ) -> Result<PaymentStatusResponse, CygnusError> {
        let res = self
            .client
            .get(&format!("{}/payments/{}/status", self.base_url, payment_id))
            .send()
            .await
            .map_err(CygnusError::Http)?;

        let payment_response: PaymentStatusResponse =
            res.json().await.map_err(CygnusError::Http)?;

        if payment_response.base.success.is_none() {
            return Err(CygnusError::Api("Invalid response received".into()));
        }

        if !payment_response.base.success.unwrap() {
            return Err(CygnusError::Api(payment_response.base.error_msg.unwrap()));
        }

        Ok(payment_response)
    }

    pub async fn get_payments(
        &self,
        payment_type: String,
    ) -> Result<PaymentListResponse, CygnusError> {
        let res = self
            .client
            .get(&format!("{}/payments/{}", self.base_url, payment_type))
            .send()
            .await
            .map_err(CygnusError::Http)?;

        let payment_list_response: PaymentListResponse =
            res.json().await.map_err(CygnusError::Http)?;

        if payment_list_response.base.success.is_none() {
            return Err(CygnusError::Api("Invalid response received".into()));
        }

        if !payment_list_response.base.success.unwrap() {
            return Err(CygnusError::Api(
                payment_list_response.base.error_msg.unwrap(),
            ));
        }

        Ok(payment_list_response)
    }
}
