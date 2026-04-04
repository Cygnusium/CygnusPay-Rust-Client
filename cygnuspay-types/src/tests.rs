#[cfg(test)]
mod tests {
    use crate::donation_request::DonationRequestResponse;
    use crate::payment_list::{PaymentListItem, PaymentListResponse};
    use crate::payment_request::{ExpiryUnit, PaymentRequest, PaymentRequestResponse};
    use crate::payment_status::{Deposit, PaymentStatusResponse};
    use crate::shared::Status;
    use chrono::{DateTime, Utc};
    use hex;
    use serde_json::{Map, json};
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
    fn test_payment_status_deserialisation() {
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
        resp = serde_json::from_value(json_response).unwrap();
        assert_eq!(
            resp.base.error_msg,
            Some(String::from("Test error message"))
        );
        assert_eq!(resp.payment_type, Some(String::from("onetime")));
        assert!(resp.deposits.is_none());

        json_response = json!(
            {
                "success": true,
                "deposits": []
            }
        );
        resp = serde_json::from_value(json_response).unwrap();
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
        resp = serde_json::from_value(json_response).unwrap();
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
        resp = serde_json::from_value(json_response.clone()).unwrap();

        assert_eq!(resp.base.success, Some(true));
        assert_eq!(resp.currency, Some("USDT".into()));

        assert!(resp.deposits.is_some());
        let deposits: Vec<Deposit> = resp.deposits.unwrap();

        assert_eq!(deposits.len(), 3);
        // Check 1 deposit
        let deposit: &Deposit = &deposits[1];
        assert_eq!(deposit.amount, 6.0);
        assert_eq!(
            deposit.timestamp,
            "2026-02-28T03:15:30Z".parse::<DateTime<Utc>>().unwrap()
        );
        assert_eq!(
            deposit.tx_hash,
            String::from("9e6c4e1a72d3f9b2c3d4a6f7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7")
        );

        assert!(resp.metadata.is_some());
        let binding = resp.metadata.unwrap();
        let map: &Map<String, serde_json::Value> = binding.as_object().unwrap();
        assert_eq!(map.keys().len(), 0);
        assert!(matches!(resp.status.unwrap(), Status::INACTIVE));
    }

    #[test]
    fn test_payment_list_item_deserialisation() {
        let mut json_response = json!({});
        assert!(serde_json::from_value::<PaymentListItem>(json_response.clone()).is_err());

        json_response = json!({
            "status": "inactive",
            "is_expired": false,
            "expires_at": "Invalid",
            "created_at": "Timestamp"
        });
        assert!(serde_json::from_value::<PaymentListItem>(json_response.clone()).is_err());

        // full response
        json_response = json!(
            {
                "status": "active",
                "title": "Test payment",
                "currency": "USDT",
                "id": "cns_payment_88381264341dawd212",
                "is_expired": false,
                "created_at": "2026-06-25T11:45:21Z",
                "expires_at": "2026-10-18T03:09:55Z",
                "type": "onetime",
                "url": "https://example.com/payment",
            }
        );
        assert!(serde_json::from_value::<PaymentListItem>(json_response.clone()).is_ok());
        let resp: PaymentListItem = serde_json::from_value(json_response).unwrap();

        assert!(matches!(resp.status, Status::ACTIVE));
        assert_eq!(resp.title, "Test payment");
        assert_eq!(resp.currency, String::from("USDT"));
        assert_eq!(resp.id, String::from("cns_payment_88381264341dawd212"));
        assert_eq!(
            resp.created_at,
            "2026-06-25T11:45:21Z".parse::<DateTime<Utc>>().unwrap()
        );
        assert_eq!(
            resp.expires_at,
            "2026-10-18T03:09:55Z".parse::<DateTime<Utc>>().unwrap()
        );
        assert_eq!(resp.payment_type, String::from("onetime"));
        assert_eq!(resp.url, String::from("https://example.com/payment"));
    }

    #[test]
    fn test_payment_list_deserialisation() {
        let mut json_response = json!({});
        assert!(serde_json::from_value::<PaymentListResponse>(json_response.clone()).is_ok());

        json_response = json!({
            "success": false,
            "error": "Test error",
            "payments": [],
        });

        assert!(serde_json::from_value::<PaymentListResponse>(json_response.clone()).is_ok());
        let mut resp: PaymentListResponse = serde_json::from_value(json_response.clone()).unwrap();
        assert_eq!(resp.base.error_msg, Some(String::from("Test error")));
        assert!(resp.items.is_some());
        assert_eq!(resp.items.unwrap().len(), 0);

        // full response
        json_response = json!({
            "success": true,
            "count": 3,
            "payments": [
                {
                    "status": "active",
                    "title": "Test payment 1",
                    "currency": "USDT",
                    "id": "cns_payment_4883812641dawd2123",
                    "is_expired": false,
                    "created_at": "2026-02-14T08:22:15Z",
                    "expires_at": "2026-05-20T14:05:59Z",
                    "type": "onetime",
                    "url": "https://example.com/payment1",
                },
                {
                    "status": "inactive",
                    "title": "Test payment 2",
                    "currency": "BTC",
                    "id": "cns_payment_8wd2142388126431da",
                    "is_expired": false,
                    "created_at": "2026-07-01T23:12:01Z",
                    "expires_at": "2026-08-30T03:45:10Z",
                    "type": "onetime",
                    "url": "https://example.com/payment2",
                },
                {
                    "status": "active",
                    "title": "Test payment 3",
                    "currency": "CNS",
                    "id": "cns_payment_8818d21312a26434wd",
                    "is_expired": false,
                    "created_at": "2026-10-15T19:00:23Z",
                    "expires_at": "2026-12-25T11:30:00Z",
                    "type": "onetime",
                    "url": "https://example.com/payment3",
                }
            ],
        });

        resp = serde_json::from_value(json_response).unwrap();

        assert_eq!(resp.base.success, Some(true));
        assert_eq!(resp.count, Some(3));
        assert!(resp.items.is_some());
        assert_eq!(resp.items.unwrap().len(), 3);
    }

    #[test]
    fn test_payment_request_response_deserialisation() {
        let mut json_response = json!({});
        assert!(serde_json::from_value::<PaymentRequestResponse>(json_response.clone()).is_ok());

        json_response = json!(
            {
                "success": false,
                "error": "Test error",
            }
        );

        assert!(serde_json::from_value::<PaymentRequestResponse>(json_response.clone()).is_ok());
        let mut resp: PaymentRequestResponse = serde_json::from_value(json_response).unwrap();
        assert_eq!(resp.base.success, Some(false));
        assert_eq!(resp.base.error_msg, Some(String::from("Test error")));

        json_response = json!(
            {
                "success": true,
                "payment_id": "cns_payment_8818d21312a26434wd",
                "payment_url": "https://example.com/payment",
                "amount": 10.5,
                "currency": "USDT",
            }
        );

        resp = serde_json::from_value(json_response).unwrap();
        assert_eq!(resp.base.success, Some(true));
        assert_eq!(
            resp.payment_id,
            Some(String::from("cns_payment_8818d21312a26434wd"))
        );
        assert_eq!(
            resp.payment_url,
            Some(String::from("https://example.com/payment"))
        );
        assert_eq!(resp.amount, Some(10.5));
        assert_eq!(resp.currency, Some(String::from("USDT")));
    }

    #[test]
    fn test_donation_request_response_deserialisation() {
        let mut json_response = json!({});
        assert!(serde_json::from_value::<DonationRequestResponse>(json_response.clone()).is_ok());

        json_response = json!(
            {
                "success": false,
                "error": "Test error",
            }
        );

        assert!(serde_json::from_value::<DonationRequestResponse>(json_response.clone()).is_ok());
        let mut resp: DonationRequestResponse = serde_json::from_value(json_response).unwrap();
        assert_eq!(resp.base.success, Some(false));
        assert_eq!(resp.base.error_msg, Some(String::from("Test error")));

        json_response = json!(
            {
                "success": true,
                "payment_id": "cns_payment_8818d21312a26434wd",
                "payment_url": "https://example.com/payment",
                "amount": 10.5,
                "currency": "USDT",
            }
        );

        resp = serde_json::from_value(json_response).unwrap();
        assert_eq!(resp.base.success, Some(true));
        assert_eq!(
            resp.payment_id,
            Some(String::from("cns_payment_8818d21312a26434wd"))
        );
        assert_eq!(
            resp.payment_url,
            Some(String::from("https://example.com/payment"))
        );
        assert_eq!(resp.amount, Some(10.5));
        assert_eq!(resp.currency, Some(String::from("USDT")));
    }
}
