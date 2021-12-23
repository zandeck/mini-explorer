use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::data::shelley::{Certificate, Mint, Update, ValidityInterval};
use crate::data::{TxIn, Value};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxBodyAlonzo {
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOutAlonzo>,
    pub certificates: Vec<Certificate>,
    pub withdrawals: HashMap<String, u64>,
    pub fee: u64,
    #[serde(rename = "validityInterval")]
    pub validity_interval: ValidityInterval,
    pub update: Option<Update>,
    pub mint: Mint,
    pub network: Option<u64>,
    #[serde(rename = "scriptIntegrityHash")]
    pub script_integrity_hash: Option<String>,
    #[serde(rename = "requiredExtraSignatures")]
    pub required_extra_signatures: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxOutAlonzo {
    pub address: String,
    pub value: Value,
    pub datum: Option<String>,
}
