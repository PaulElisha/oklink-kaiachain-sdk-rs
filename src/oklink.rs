use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use crate::types::ProtocolType;

const BASE_URL: &str = "https://www.oklink.com/";
const CHAIN_SHORT_NAME: &str = "KLAYTN";

#[derive(Debug, Deserialize)]
pub struct AddressInformation {
    code: String,
    msg: String,
    data: AddressData,
}

#[derive(Debug, Deserialize)]
pub struct AddressData {
    chain_full_name: String,
    chain_short_name: String,
    address: String,
    contract_address: String,
    balance: String,
    balance_symbol: String,
    transaction_count: String,
    verifying: String,
    send_amount: String,
    receive_amount: String,
    token_amount: String,
    total_token_value: String,
    create_contract_address: String,
    create_contract_transaction_hash: String,
    first_transaction_time: String,
    last_transaction_time: String,
    token: String,
    bandwidth: String,
    energy: String,
    voting_rights: String,
    unclaimed_voting_rewards: String,
    is_aa_address: bool,
}

pub struct Oklink {
    api_key: String,
    client: Client,
}

impl Oklink {
    pub fn new(api_key: String) -> Self {
        Oklink {
            api_key,
            client: Client::new(),
        }
    }

    fn headers(&self) -> HashMap<&str, &str> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type", "application/json");
        headers.insert("Ok-Access-Key", &self.api_key);
        headers
    }

    async fn _get<T: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> Result<T, reqwest::Error> {
        let url = format!("{}{}", BASE_URL, endpoint);
        let response = self
            .client
            .get(&url)
            .headers(self.headers().into())
            .query(&params)
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(response)
    }

    pub async fn address_info(&self, address: &str) -> Result<AddressInformation, reqwest::Error> {
        let params = [("chainShortName", CHAIN_SHORT_NAME), ("address", address)];
        self._get("api/v5/explorer/address/address-summary", &params).await
    }

    pub async fn evm_address_info(
        &self,
        address: &str,
    ) -> Result<AddressInformation, reqwest::Error> {
        let params = [("chainShortName", CHAIN_SHORT_NAME), ("address", address)];
        let url = format!("{}api/v5/explorer/address/information-evm", BASE_URL);
        let response = self
            .client
            .get(&url)
            .headers(self.headers().into())
            .query(&params)
            .send()
            .await?
            .json::<AddressInformation>()
            .await?;
        Ok(response)
        self._get("api/v5/explorer/address/information-evm", &params).await
    }

    pub async fn address_active_chain(&self, address: &str) -> Result<AddressInformation, reqwest::Error> {
        let params = [("chainShortName", CHAIN_SHORT_NAME), ("address", address)];
        self._get("api/v5/explorer/address/address-active-chain", &params).await
    }

    pub async fn address_token_balance(&self, address: &str, protocol_type: ProtocolType, token_contract_address: Option<&str>, page: Option<&str>, limit: Option<&str>) -> Result<AddressInformation, reqwest::Error> {
        let mut params = vec![
            ("chainShortName", CHAIN_SHORT_NAME),
            ("address", address),
            ("protocolType", &serde_json::to_string(&protocol_type).unwrap()),
        ];
        if let Some(token_contract_address) = token_contract_address {
            params.push(("tokenContractAddress", token_contract_address));
        }
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/address/token-balance", &params).await
    }

    pub async fn address_balance_details(
        &self,
        address: &str,
        protocol_type: &str,
        token_contract_address: Option<&str>,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        let mut params = vec![
            ("chainShortName", CHAIN_SHORT_NAME),
            ("address", address),
            ("protocolType", protocol_type),
        ];
        if let Some(token_contract_address) = token_contract_address {
            params.push(("tokenContractAddress", token_contract_address));
        }
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/address/address-balance-fills", &params).await
    }

    pub async fn address_balance_history(
        &self,
        address: &str,
        height: &str,
        token_contract_address: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        let mut params = vec![
            ("chainShortName", CHAIN_SHORT_NAME),
            ("address", address),
            ("height", height),
        ];
        if let Some(token_contract_address) = token_contract_address {
            params.push(("tokenContractAddress", token_contract_address));
        }
        self._get("api/v5/explorer/block/address-balance-history", &params).await
    }

    pub async fn address_transaction_list(
        &self,
        address: &str,
        protocol_type: Option<&str>,
        symbol: Option<&str>,
        start_block_height: Option<&str>,
        end_block_height: Option<&str>,
        is_from_or_to: Option<&str>,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        let mut params = vec![
            ("chainShortName", CHAIN_SHORT_NAME),
            ("address", address),
        ];
        if let Some(protocol_type) = protocol_type {
            params.push(("protocolType", protocol_type));
        }
        if let Some(symbol) = symbol {
            params.push(("symbol", symbol));
        }
        if let Some(start_block_height) = start_block_height {
            params.push(("startBlockHeight", start_block_height));
        }
        if let Some(end_block_height) = end_block_height {
            params.push(("endBlockHeight", end_block_height));
        }
        if let Some(is_from_or_to) = is_from_or_to {
            params.push(("isFromOrTo", is_from_or_to));
        }
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/address/transaction-list", &params).await
    }

    pub async fn address_normal_transaction_list(
        &self,
        address: &str,
        start_block_height: Option<&str>,
        end_block_height: Option<&str>,
        is_from_or_to: Option<&str>,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        let mut params = vec![("chainShortName", CHAIN_SHORT_NAME), ("address", address)];
        if let Some(start_block_height) = start_block_height {
            params.push(("startBlockHeight", start_block_height));
        }
        if let Some(end_block_height) = end_block_height {
            params.push(("endBlockHeight", end_block_height));
        }
        if let Some(is_from_or_to) = is_from_or_to {
            params.push(("isFromOrTo", is_from_or_to));
        }
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/address/normal-transaction-list", &params).await
    }

    pub async fn address_internal_transaction_list(
        &self,
        address: &str,
        start_block_height: Option<&str>,
        end_block_height: Option<&str>,
        is_from_or_to: Option<&str>,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        let mut params = vec![("chainShortName", CHAIN_SHORT_NAME), ("address", address)];
        if let Some(start_block_height) = start_block_height {
            params.push(("startBlockHeight", start_block_height));
        }
        if let Some(end_block_height) = end_block_height {
            params.push(("endBlockHeight", end_block_height));
        }
        if let Some(is_from_or_to) = is_from_or_to {
            params.push(("isFromOrTo", is_from_or_to));
        }
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/address/internal-transaction-list", &params).await
    }

    pub async fn address_token_transaction_list(
        &self,
        address: &str,
        protocol_type: &str,
        token_contract_address: Option<&str>,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        let mut params = vec![
            ("chainShortName", CHAIN_SHORT_NAME),
            ("address", address),
            ("protocolType", protocol_type),
        ];
        if let Some(token_contract_address) = token_contract_address {
            params.push(("tokenContractAddress", token_contract_address));
        }
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/address/token-transaction-list", &params).await
    }

    pub async fn address_entity_labels(
        &self,
        address: &str,
    ) -> Result<AddressInformation, reqwest::Error> {
        let params = [("chainShortName", CHAIN_SHORT_NAME), ("address", address)];
        self._get("api/v5/explorer/address/entity-labels", &params).await
    }

    pub async fn batch_address_balances(
        &self,
        addresses: &[&str],
    ) -> Result<AddressInformation, reqwest::Error> {
        if addresses.len() > 100 {
            return Err(reqwest::Error::new(
                reqwest::StatusCode::BAD_REQUEST,
                "The maximum number of addresses is 100",
            ));
        }
        let params = [
            ("chainShortName", CHAIN_SHORT_NAME),
            ("addresses", &addresses.join(",")),
        ];
        self._get("api/v5/explorer/address/balance-multi", &params).await
    }

    pub async fn batch_address_token_balances(
        &self,
        addresses: &[&str],
        protocol_type: Option<&str>,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        if addresses.len() > 50 {
            return Err(reqwest::Error::new(
                reqwest::StatusCode::BAD_REQUEST,
                "The maximum number of addresses is 50",
            ));
        }
        let mut params = vec![
            ("chainShortName", CHAIN_SHORT_NAME),
            ("addresses", &addresses.join(",")),
        ];
        if let Some(protocol_type) = protocol_type {
            params.push(("protocolType", protocol_type));
        }
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/address/token-balance-multi", &params).await
    }

    pub async fn batch_address_normal_transaction_list(
        &self,
        addresses: &[&str],
        start_block_height: Option<&str>,
        end_block_height: Option<&str>,
        is_from_or_to: Option<&str>,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        if addresses.len() > 50 {
            return Err(reqwest::Error::new(
                reqwest::StatusCode::BAD_REQUEST,
                "The maximum number of addresses is 50",
            ));
        }
        let mut params = vec![
            ("chainShortName", CHAIN_SHORT_NAME),
            ("addresses", &addresses.join(",")),
        ];
        if let Some(start_block_height) = start_block_height {
            params.push(("startBlockHeight", start_block_height));
        }
        if let Some(end_block_height) = end_block_height {
            params.push(("endBlockHeight", end_block_height));
        }
        if let Some(is_from_or_to) = is_from_or_to {
            params.push(("isFromOrTo", is_from_or_to));
        }
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/address/normal-transaction-list-multi", &params).await
    }

    pub async fn batch_address_internal_transaction_list(
        &self,
        addresses: &[&str],
        start_block_height: Option<&str>,
        end_block_height: Option<&str>,
        is_from_or_to: Option<&str>,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        if addresses.len() > 20 {
            return Err(reqwest::Error::new(
                reqwest::StatusCode::BAD_REQUEST,
                "The maximum number of addresses is 20",
            ));
        }
        let mut params = vec![
            ("chainShortName", CHAIN_SHORT_NAME),
            ("addresses", &addresses.join(",")),
        ];
        if let Some(start_block_height) = start_block_height {
            params.push(("startBlockHeight", start_block_height));
        }
        if let Some(end_block_height) = end_block_height {
            params.push(("endBlockHeight", end_block_height));
        }
        if let Some(is_from_or_to) = is_from_or_to {
            params.push(("isFromOrTo", is_from_or_to));
        }
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        let url = format!(
            "{}api/v5/explorer/address/internal-transaction-list-multi",
            BASE_URL
        );
        let response = self
            .client
            .get(&url)
            .headers(self.headers().into())
            .query(&params)
            .send()
            .await?
            .json::<AddressInformation>()
            .await?;
        Ok(response)
        self._get("api/v5/explorer/address/internal-transaction-list-multi", &params).await
    }

    pub async fn batch_address_token_transaction_list(
        &self,
        addresses: &[&str],
        start_block_height: &str,
        end_block_height: &str,
        page: Option<&str>,
        limit: Option<&str>,
        protocol_type: Option<&str>,
        token_contract_address: Option<&str>,
        is_from_or_to: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        if addresses.len() > 20 {
            return Err(reqwest::Error::new(
                reqwest::StatusCode::BAD_REQUEST,
                "The maximum number of addresses is 20",
            ));
        }
        let mut params = vec![
            ("chainShortName", CHAIN_SHORT_NAME),
            ("addresses", &addresses.join(",")),
            ("startBlockHeight", start_block_height),
            ("endBlockHeight", end_block_height),
        ];
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        if let Some(protocol_type) = protocol_type {
            params.push(("protocolType", protocol_type));
        }
        if let Some(token_contract_address) = token_contract_address {
            params.push(("tokenContractAddress", token_contract_address));
        }
        if let Some(is_from_or_to) = is_from_or_to {
            params.push(("isFromOrTo", is_from_or_to));
        }
        self._get("api/v5/explorer/address/token-transaction-list-multi", &params).await
    }

    pub async fn rich_list(
        &self,
        address: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        let mut params = vec![("chainShortName", CHAIN_SHORT_NAME)];
        if let Some(address) = address {
            params.push(("address", address));
        }
        self._get("api/v5/explorer/address/rich-list", &params).await
    }

    pub async fn native_token_ranking(
        &self,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        let mut params = vec![("chainShortName", CHAIN_SHORT_NAME)];
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/address/native-token-position-list", &params).await
    }

    pub async fn transaction_list(
        &self,
        block_hash: Option<&str>,
        height: Option<&str>,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        let mut params = vec![("chainShortName", CHAIN_SHORT_NAME)];
        if let Some(block_hash) = block_hash {
            params.push(("blockHash", block_hash));
        }
        if let Some(height) = height {
            params.push(("height", height));
        }
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/transaction/transaction-list", &params).await
    }

    pub async fn large_transaction_list(
        &self,
        transaction_type: Option<&str>,
        height: Option<&str>,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        let mut params = vec![("chainShortName", CHAIN_SHORT_NAME)];
        if let Some(transaction_type) = transaction_type {
            params.push(("type", transaction_type));
        }
        if let Some(height) = height {
            params.push(("height", height));
        }
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/transaction/large-transaction-list", &params).await
    }

    pub async fn unconfirmed_transaction_list(
        &self,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        let mut params = vec![("chainShortName", CHAIN_SHORT_NAME)];
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/transaction/unconfirmed-transaction-list", &params).await
    }

    pub async fn internal_transaction_details(
        &self,
        tx_id: &str,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        let mut params = vec![("chainShortName", CHAIN_SHORT_NAME), ("txId", tx_id)];
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/transaction/internal-transaction-detail", &params).await
    }

    pub async fn token_transaction_details(
        &self,
        tx_id: &str,
        protocol_type: &str,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        let mut params = vec![("chainShortName", CHAIN_SHORT_NAME), ("txId", tx_id)];
        if let Some(protocol_type) = protocol_type {
            params.push(("protocolType", protocol_type));
        }
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/transaction/token-transaction-detail", &params).await
    }

    pub async fn transaction_details(
        &self,
        tx_id: &str,
    ) -> Result<AddressInformation, reqwest::Error> {
        let params = [("chainShortName", CHAIN_SHORT_NAME), ("txId", tx_id)];
        self._get("api/v5/explorer/transaction/transaction-fills", &params).await
    }

    pub async fn batch_transaction_details(
        &self,
        tx_ids: &[&str],
    ) -> Result<AddressInformation, reqwest::Error> {
        if tx_ids.len() > 20 {
            return Err(reqwest::Error::new(
                reqwest::StatusCode::BAD_REQUEST,
                "The maximum number of transactions is 20",
            ));
        }
        let params = [
            ("chainShortName", CHAIN_SHORT_NAME),
            ("txIds", &tx_ids.join(",")),
        ];
        self._get("api/v5/explorer/transaction/transaction-multi", &params).await
    }

    pub async fn batch_internal_transaction_details(
        &self,
        tx_ids: &[&str],
    ) -> Result<AddressInformation, reqwest::Error> {
        if tx_ids.len() > 20 {
            return Err(reqwest::Error::new(
                reqwest::StatusCode::BAD_REQUEST,
                "The maximum number of transactions is 20",
            ));
        }
        let params = [
            ("chainShortName", CHAIN_SHORT_NAME),
            ("txIds", &tx_ids.join(",")),
        ];
        self._get("api/v5/explorer/transaction/internal-transaction-multi", &params).await
    }

    pub async fn batch_token_transaction_details(
        &self,
        tx_ids: &[&str],
        protocol_type: Option<&str>,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        if tx_ids.len() > 20 {
            return Err(reqwest::Error::new(
                reqwest::StatusCode::BAD_REQUEST,
                "The maximum number of transactions is 20",
            ));
        }
        let mut params = vec![
            ("chainShortName", CHAIN_SHORT_NAME),
            ("txIds", &tx_ids.join(",")),
        ];
        if let Some(protocol_type) = protocol_type {
            params.push(("protocolType", protocol_type));
        }
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/transaction/token-transfer-multi", &params).await
    }

    pub async fn token_list(
        &self,
        protocol_type: Option<&str>,
        token_contract_address: Option<&str>,
        start_time: Option<&str>,
        end_time: Option<&str>,
        order_by: Option<&str>,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        let mut params = vec![("chainShortName", CHAIN_SHORT_NAME)];
        if let Some(protocol_type) = protocol_type {
            params.push(("protocolType", protocol_type));
        }
        if let Some(token_contract_address) = token_contract_address {
            params.push(("tokenContractAddress", token_contract_address));
        }
        if let Some(start_time) = start_time {
            params.push(("startTime", start_time));
        }
        if let Some(end_time) = end_time {
            params.push(("endTime", end_time));
        }
        if let Some(order_by) = order_by {
            params.push(("orderBy", order_by));
        }
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/token/token-list", &params).await
    }

    pub async fn token_position_list(
        &self,
        token_contract_address: &str,
        holder_address: Option<&str>,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        let mut params = vec![
            ("chainShortName", CHAIN_SHORT_NAME),
            ("tokenContractAddress", token_contract_address),
        ];
        if let Some(holder_address) = holder_address {
            params.push(("holderAddress", holder_address));
        }
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/token/position-list", &params).await
    }

    pub async fn token_position_statistics(
        &self,
        token_contract_address: &str,
        holder_address: Option<&str>,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        let mut params = vec![
            ("chainShortName", CHAIN_SHORT_NAME),
            ("tokenContractAddress", token_contract_address),
        ];
        if let Some(holder_address) = holder_address {
            params.push(("holderAddress", holder_address));
        }
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/token/position-statistics", &params).await
    }

    pub async fn token_transfer_details(
        &self,
        token_contract_address: &str,
        max_amount: Option<&str>,
        min_amount: Option<&str>,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        let mut params = vec![
            ("chainShortName", CHAIN_SHORT_NAME),
            ("tokenContractAddress", token_contract_address),
        ];
        if let Some(max_amount) = max_amount {
            params.push(("maxAmount", max_amount));
        }
        if let Some(min_amount) = min_amount {
            params.push(("minAmount", min_amount));
        }
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/token/transaction-list", &params).await
    }

    pub async fn batch_token_transaction(
        &self,
        token_contract_address: &str,
        start_block_height: &str,
        end_block_height: &str,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        let mut params = vec![
            ("chainShortName", CHAIN_SHORT_NAME),
            ("tokenContractAddress", token_contract_address),
            ("startBlockHeight", start_block_height),
            ("endBlockHeight", end_block_height),
        ];
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/token/token-transaction-list-multi", &params).await
    }

    pub async fn token_supply_history(
        &self,
        token_contract_address: &str,
        height: &str,
    ) -> Result<AddressInformation, reqwest::Error> {
        let params = [
            ("chainShortName", CHAIN_SHORT_NAME),
            ("tokenContractAddress", token_contract_address),
            ("height", height),
        ];
        self._get("api/v5/explorer/token/supply-history", &params).await
    }

    pub async fn token_transaction_statistics(
        &self,
        token_contract_address: &str,
        order_by: Option<&str>,
        page: Option<&str>,
        limit: Option<&str>,
    ) -> Result<AddressInformation, reqwest::Error> {
        let mut params = vec![
            ("chainShortName", CHAIN_SHORT_NAME),
            ("tokenContractAddress", token_contract_address),
        ];
        if let Some(order_by) = order_by {
            params.push(("orderBy", order_by));
        }
        if let Some(page) = page {
            params.push(("page", page));
        }
        if let Some(limit) = limit {
            params.push(("limit", limit));
        }
        self._get("api/v5/explorer/token/transaction-stats", &params).await
    }
}