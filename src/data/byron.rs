use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::data::{ProtocolVersion, Tx, TxIn, TxOut};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ByronBlockEra<BkHeader, TxBody>
where
    TxBody: Clone,
{
    #[serde(rename = "txPayload")]
    pub body: Option<ByronBody<TxBody>>,
    pub header: BkHeader,
    #[serde(rename = "hash")]
    pub header_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ByronBody<TxBody>
where
    TxBody: Clone,
{
    #[serde(rename = "txPayload")]
    pub tx_payload: Option<Vec<Tx<TxBody>>>,
    #[serde(rename = "updatePayload")]
    pub update_payload: HashMap<String, Option<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxBodyByron {
    pub inputs: Option<Vec<TxIn>>,
    pub outputs: Option<Vec<TxOut>>,
    pub fee: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ByronHeader {
    #[serde(rename = "protocolMagicId")]
    pub protocol_magic_id: Option<u64>,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: Option<ProtocolVersion>,
    #[serde(rename = "blockHeight")]
    pub block_height: u64,
    #[serde(rename = "prevHash")]
    pub prev_hash: String,
    pub epoch: Option<u64>,
    #[serde(rename = "softwareVersion")]
    pub software_version: Option<SoftwareVersion>,
    #[serde(rename = "genesisKey")]
    pub genesis_key: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SoftwareVersion {
    #[serde(rename = "appName")]
    pub app_name: String,
    pub number: u64,
}
