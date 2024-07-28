use oklink::Oklink;
use mockito::mock;
use tokio;

#[tokio::test]
async fn test_evm_address_info() {
    let _m = mock("GET", "/api/v5/explorer/address/information-evm")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
            "code": "0",
            "msg": "success",
            "data": {
                "chain_full_name": "Klaytn",
                "chain_short_name": "KLAYTN",
                "address": "0xYourAddress",
                "contract_address": "",
                "balance": "1000",
                "balance_symbol": "KLAY",
                "transaction_count": "10",
                "verifying": "false",
                "send_amount": "500",
                "receive_amount": "1500",
                "token_amount": "0",
                "total_token_value": "0",
                "create_contract_address": "",
                "create_contract_transaction_hash": "",
                "first_transaction_time": "2021-01-01T00:00:00Z",
                "last_transaction_time": "2021-01-02T00:00:00Z",
                "token": "",
                "bandwidth": "",
                "energy": "",
                "voting_rights": "",
                "unclaimed_voting_rewards": "",
                "is_aa_address": false
            }
        }"#)
        .create();

    let api_key = "test_api_key";
    let oklink = Oklink::new(api_key.to_string());
    let result = oklink.evm_address_info("0xYourAddress").await;

    assert!(result.is_ok());
    let info = result.unwrap();
    assert_eq!(info.code, "0");
    assert_eq!(info.msg, "success");
    assert_eq!(info.data.address, "0xYourAddress");
}

#[tokio::test]
async fn test_address_balance_details() {
    let _m = mock("GET", "/api/v5/explorer/address/address-balance-fills")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{
            "code": "0",
            "msg": "success",
            "data": {
                "chain_full_name": "Klaytn",
                "chain_short_name": "KLAYTN",
                "address": "0xYourAddress",
                "contract_address": "",
                "balance": "1000",
                "balance_symbol": "KLAY",
                "transaction_count": "10",
                "verifying": "false",
                "send_amount": "500",
                "receive_amount": "1500",
                "token_amount": "0",
                "total_token_value": "0",
                "create_contract_address": "",
                "create_contract_transaction_hash": "",
                "first_transaction_time": "2021-01-01T00:00:00Z",
                "last_transaction_time": "2021-01-02T00:00:00Z",
                "token": "",
                "bandwidth": "",
                "energy": "",
                "voting_rights": "",
                "unclaimed_voting_rewards": "",
                "is_aa_address": false
            }
        }"#)
        .create();

    let api_key = "test_api_key";
    let oklink = Oklink::new(api_key.to_string());
    let result = oklink.address_balance_details("0xYourAddress", "protocol", None, None, None).await;

    assert!(result.is_ok());
    let info = result.unwrap();
    assert_eq!(info.code, "0");
    assert_eq!(info.msg, "success");
    assert_eq!(info.data.address, "0xYourAddress");
}