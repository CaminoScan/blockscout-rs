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

#[derive(new, Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InternalTransaction {
    #[serde(rename = "block")]
    pub block: i32,
    #[serde(rename = "created_contract")]
    pub created_contract: Option<models::AddressParam>,
    #[serde(rename = "error")]
    pub error: Option<String>,
    #[serde(rename = "from")]
    pub from: models::AddressParam,
    #[serde(rename = "gas_limit")]
    pub gas_limit: String,
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "to")]
    pub to: models::AddressParam,
    #[serde(rename = "transaction_hash")]
    pub transaction_hash: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "value")]
    pub value: String,
}
