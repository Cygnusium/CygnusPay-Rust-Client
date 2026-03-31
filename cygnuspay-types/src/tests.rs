#[cfg(test)]
mod tests {
    use crate::payment::{Deposit, PaymentRequest, PaymentStatusResponse, ExpiryUnit, Status};
    use chrono::{DateTime, Utc};
    use hex;
    use serde_json::{json, Map};
    use sha2::{Digest, Sha256};

    #[test]
    fn test_payment_request_full_serialisation() {
        let request = PaymentRequest {
            amount: 10.5,
            currency: "USDT".into(),
            description: Some("Hello description".into()),
            expiry_unit: Some(ExpiryUnit::DAYS),
            expiry_value: Some(10),
            metadata: Some(json!({
                "yippee!": "yahoo!"
            })),
            title: Some("Title".into()),
        };

        let value = serde_json::to_value(&request).unwrap();

        assert_eq!(value["amount"], 10.5);
        assert_eq!(value["currency"], "USDT");
        assert_eq!(value["description"], "Hello description");
        assert_eq!(value["expiry_value"], 10);
        assert_eq!(value["expiry_unit"], "days");
        assert_eq!(value["metadata"]["yippee!"], "yahoo!");
        assert_eq!(value["title"], "Title");
    }

    #[test]
    fn test_payment_request_optional_fields_omitted() {
        let request = PaymentRequest {
            amount: 10.5,
            currency: "USDT".into(),
            description: None,
            expiry_unit: None,
            expiry_value: None,
            metadata: None,
            title: None,
        };

        let value = serde_json::to_value(&request).unwrap();

        // amount and currency should exist
        assert_eq!(value["amount"], 10.5);
        assert_eq!(value["currency"], "USDT");
        assert!(value.get("description").is_none());
        assert!(value.get("expiry_unit").is_none());
        assert!(value.get("expiry_value").is_none());
        assert!(value.get("metadata").is_none());
        assert!(value.get("title").is_none());
    }

    #[test]
    fn test_payment_request_constructor_defaults() {
        let request = PaymentRequest::new(10.5, "USDT".into());

        let value = serde_json::to_value(&request).unwrap();

        // amount should exist
        assert_eq!(value["amount"], 10.5);
        assert_eq!(value["currency"], "USDT");
        assert!(value.get("description").is_none());
        assert!(value.get("expiry_unit").is_none());
        assert!(value.get("expiry_value").is_none());
        assert!(value.get("metadata").is_none());
        assert!(value.get("title").is_none());
    }

    #[test]
    fn test_payment_request_setters_update_fields() {
        let mut request = PaymentRequest::new(10.5, "USDT".into());

        request.set_description(Some("Hello description".into()));
        request.set_metadata(Some(json!({"yippee!": "yahoo!"})));
        request.set_title(Some("Title".into()));
        request.set_expiry_unit(Some(ExpiryUnit::MINUTES));

        let mut value = serde_json::to_value(&request).unwrap();
        assert_eq!(value["amount"], 10.5);
        assert_eq!(value["currency"], "USDT");
        assert_eq!(value["description"], "Hello description");
        assert_eq!(value["expiry_unit"], "minutes");
        assert_eq!(value["metadata"]["yippee!"], "yahoo!");
        assert_eq!(value["title"], "Title");

        request.set_metadata(None);
        value = serde_json::to_value(&request).unwrap();
        assert!(value.get("metadata").is_none());
    }

    #[test]
    fn test_deposit_struct_deserialisation() {
        let mut hasher = Sha256::new();
        hasher.update(b"test data");
        let hex_result = hex::encode(hasher.finalize());

        let mut deposit_json = json!({});
        assert!(serde_json::from_value::<Deposit>(deposit_json.clone()).is_err());

        deposit_json = json!({
            "amount": 10.5,
            "timestamp": "Invalid timestamp",
            "tx_hash": hex_result
        });
        // error with deserialising timestamp
        assert!(serde_json::from_value::<Deposit>(deposit_json.clone()).is_err());

        let timestamp = Utc::now();
        deposit_json = json!({
            "amount": 10.5,
            "timestamp": timestamp.to_string(),
            "tx_hash": hex_result
        });
        assert!(serde_json::from_value::<Deposit>(deposit_json.clone()).is_ok());
        let resp = serde_json::from_value::<Deposit>(deposit_json.clone()).unwrap();

        assert_eq!(10.5, resp.amount);
        assert_eq!(timestamp, resp.timestamp);
        assert_eq!(hex_result, resp.tx_hash);
    }

    #[test]
    fn test_payment_response_deserialisation() {
        let mut json_response = json!({});
        // passes because all fields are optional
        assert!(serde_json::from_value::<PaymentStatusResponse>(json_response.clone()).is_ok());

        json_response = json!({
            "success": true,
            "invalid_field": "invalid!!!"
        });
        assert!(serde_json::from_value::<PaymentStatusResponse>(json_response.clone()).is_ok());

        let mut resp: PaymentStatusResponse = serde_json::from_value(json_response).unwrap();
        assert_eq!(resp.base.success, Some(true));

        // 'error' key should be mapped to error_msg field on struct
        // 'type' key should be mapped to confirmed field on struct
        json_response = json!({
            "success": false,
            "error": "Test error message",
            "type": "onetime"
        });
        resp = serde_json::from_value::<PaymentStatusResponse>(json_response).unwrap();
        assert_eq!(resp.base.error_msg, Some(String::from("Test error message")));
        assert_eq!(resp.payment_type, Some(String::from("onetime")));
        assert!(resp.deposits.is_none());

        json_response = json!(
            {
                "success": true,
                "deposits": []
            }
        );
        resp = serde_json::from_value::<PaymentStatusResponse>(json_response).unwrap();
        assert!(!resp.deposits.is_none());
        assert!(resp.deposits.unwrap().is_empty());

        let mut hasher = Sha256::new();
        hasher.update(b"test data");
        let hex_result = hex::encode(hasher.finalize());

        json_response = json!({
            "success": true,
            "deposits": [
                {
                    "amount": 10.5,
                    "timestamp": Utc::now().to_string(),
                    "tx_hash": hex_result
                }
            ]
        });
        resp = serde_json::from_value::<PaymentStatusResponse>(json_response).unwrap();
        assert!(!resp.deposits.is_none());
        assert_eq!(1, resp.deposits.unwrap().len());

        // full response
        json_response = json!({
            "success": true,
            "confirmed_amount": 10000.52,
            "currency": "USDT",
            "deposits": [
                {
                    "amount": 10.5,
                    "timestamp": "2026-08-14T11:42:09Z",
                    "tx_hash": "1c322b6441369d75b75f85e5095d985a499313d4b6d41a87e5b61c5614147eaa"
                },
                {
                    "amount": 6,
                    "timestamp": "2026-02-28T03:15:30Z",
                    "tx_hash": "9e6c4e1a72d3f9b2c3d4a6f7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7"
                },
                {
                    "amount": 1234212.43123,
                    "timestamp": "2026-11-05T20:55:17Z",
                    "tx_hash": "f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9"
                }
            ],
            "expires_at": "2026-09-01T22:45:10Z",
            "id": "cns_payment_38asd942dad22ndaw832nd8211",
            "is_expired": true,
            "is_confirmed": true,
            "metadata": {},
            "payment_type": "onetime",
            "status": "inactive",
        });

        assert!(serde_json::from_value::<PaymentStatusResponse>(json_response.clone()).is_ok());
        resp = serde_json::from_value::<PaymentStatusResponse>(json_response.clone()).unwrap();

        assert_eq!(resp.base.success, Some(true));
        assert_eq!(resp.currency, Some("USDT".into()));

        assert!(resp.deposits.is_some());
        let deposits: Vec<Deposit> = resp.deposits.unwrap();

        assert_eq!(deposits.len(), 3);
        // Check 1 deposit
        let deposit: &Deposit = &deposits[1];
        assert_eq!(deposit.amount, 6.0);
        assert_eq!(deposit.timestamp, "2026-02-28T03:15:30Z".parse::<DateTime<Utc>>().unwrap());
        assert_eq!(deposit.tx_hash, String::from("9e6c4e1a72d3f9b2c3d4a6f7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7"));

        assert!(resp.metadata.is_some());
        let binding = resp.metadata.unwrap();
        let map: &Map<String, serde_json::Value> = binding.as_object().unwrap();
        assert_eq!(map.keys().len(), 0);
        assert!(matches!(resp.status.unwrap(), Status::INACTIVE));

    }
}
