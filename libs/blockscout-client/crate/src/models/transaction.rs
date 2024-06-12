/*
 * BlockScout API
 *
 * API for BlockScout web app
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: you@your-company.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use derive_new::new;
use serde::{Deserialize, Serialize};
use serde_with::*;

#[serde_as]
#[derive(new, Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "fee")]
    pub fee: models::Fee,
    #[serde(rename = "gas_limit")]
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub gas_limit: i64,
    #[serde(rename = "block")]
    pub block: i64,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "method")]
    pub method: Option<String>,
    #[serde(rename = "confirmations")]
    pub confirmations: i64,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "exchange_rate")]
    pub exchange_rate: String,
    #[serde(rename = "to")]
    pub to: models::AddressParam,
    #[serde(rename = "tx_burnt_fee")]
    pub tx_burnt_fee: String,
    #[serde(rename = "max_fee_per_gas")]
    pub max_fee_per_gas: String,
    #[serde(rename = "result")]
    pub result: String,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "gas_price")]
    pub gas_price: String,
    #[serde(rename = "priority_fee")]
    pub priority_fee: String,
    #[serde(rename = "base_fee_per_gas")]
    pub base_fee_per_gas: String,
    #[serde(rename = "from")]
    pub from: models::AddressParam,
    #[serde(rename = "token_transfers")]
    #[serde_as(deserialize_as = "DefaultOnNull")]
    pub token_transfers: Vec<models::TokenTransfer>,
    #[serde(rename = "tx_types")]
    pub tx_types: Vec<String>,
    #[serde(rename = "gas_used")]
    pub gas_used: String,
    #[serde(rename = "created_contract")]
    pub created_contract: Option<models::AddressParam>,
    #[serde(rename = "position")]
    pub position: i32,
    #[serde(rename = "nonce")]
    pub nonce: i32,
    #[serde(rename = "has_error_in_internal_txs")]
    pub has_error_in_internal_txs: Option<bool>,
    #[serde(rename = "actions")]
    pub actions: Vec<models::TransactionAction>,
    #[serde(rename = "decoded_input")]
    pub decoded_input: Option<models::DecodedInput>,
    #[serde(rename = "token_transfers_overflow")]
    pub token_transfers_overflow: Option<bool>,
    #[serde(rename = "raw_input")]
    pub raw_input: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "max_priority_fee_per_gas")]
    pub max_priority_fee_per_gas: String,
    #[serde(rename = "revert_reason")]
    pub revert_reason: Option<String>,
    #[serde(rename = "confirmation_duration")]
    pub confirmation_duration: serde_json::Value,
    #[serde(rename = "tx_tag")]
    pub tx_tag: Option<String>,
}
