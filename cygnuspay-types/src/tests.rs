#[cfg(test)]
mod tests {
    use serde_json::json;
    use chrono::Utc;
    use crate::payment::{
        PaymentRequest,
        PaymentResponse,
        Deposit
    };
    use sha2::{
        Sha256,
        Digest
    };
    use hex;

    #[test]
    fn test_payment_request_full_serialisation() {
        let request = PaymentRequest {
            amount: 10.5,
            currency: Some("USDT".into()),
            description: Some("Hello description".into()),
            metadata: Some(json!({
                "yippee!": "yahoo!"
            })),
            title: Some("Title".into()),
        };

        let value = serde_json::to_value(&request).unwrap();

        assert_eq!(value["amount"], 10.5);
        assert_eq!(value["currency"], "USDT");
        assert_eq!(value["description"], "Hello description");
        assert_eq!(value["title"], "Title");
        assert_eq!(value["metadata"]["yippee!"], "yahoo!");
    }

    #[test]
    fn test_payment_request_optional_fields_omitted() {
        let request = PaymentRequest {
            amount: 10.5,
            currency: None,
            description: None,
            metadata: None,
            title: None,
        };

        let value = serde_json::to_value(&request).unwrap();

        // amount should exist
        assert_eq!(value["amount"], 10.5);

        assert!(value.get("currency").is_none());
        assert!(value.get("description").is_none());
        assert!(value.get("metadata").is_none());
        assert!(value.get("title").is_none());
    }

    #[test]
    fn test_payment_request_constructor_defaults() {
        let request = PaymentRequest::new(10.5);

        let value = serde_json::to_value(&request).unwrap();

        // amount should exist
        assert_eq!(value["amount"], 10.5);

        assert!(value.get("currency").is_none());
        assert!(value.get("description").is_none());
        assert!(value.get("metadata").is_none());
        assert!(value.get("title").is_none());
    }

    #[test]
    fn test_payment_request_setters_update_fields() {
        let mut request = PaymentRequest::new(10.5);

        request.set_currency(Some("USDT".into()));
        request.set_description(Some("Hello description".into()));
        request.set_metadata(Some(json!({"yippee!": "yahoo!"})));
        request.set_title(Some("Title".into()));

        let mut value = serde_json::to_value(&request).unwrap();
        assert_eq!(value["amount"], 10.5);
        assert_eq!(value["currency"], "USDT");
        assert_eq!(value["description"], "Hello description");
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
        assert!(serde_json::from_value::<PaymentResponse>(json_response.clone()).is_err());

        json_response = json!({
            "success": true,
            "invalid_field": "invalid!!!"
        });
        assert!(serde_json::from_value::<PaymentResponse>(json_response.clone()).is_ok());

        let mut resp: PaymentResponse = serde_json::from_value(json_response).unwrap();
        assert_eq!(resp.success, true);

        // 'message' key should be mapped to error_message field on struct
        // 'is_confirmed' key should be mapped to confirmed field on struct
        json_response = json!({
            "success": false,
            "message": "Test error message",
            "is_confirmed": true,
        });
        resp = serde_json::from_value::<PaymentResponse>(json_response).unwrap();
        assert_eq!(resp.error_message, Some(String::from("Test error message")));
        assert_eq!(resp.confirmed, Some(true));
        assert!(resp.deposits.is_none());

        json_response = json!(
            {
                "success": true,
                "deposits": []
            }
        );
        resp = serde_json::from_value::<PaymentResponse>(json_response).unwrap();
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
        resp = serde_json::from_value::<PaymentResponse>(json_response).unwrap();
        assert!(!resp.deposits.is_none());
        assert_eq!(1, resp.deposits.unwrap().len());

    }

}