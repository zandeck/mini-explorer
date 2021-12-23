use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::data::{ProtocolVersion, Tx, TxIn, TxOut};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShelleyBlockEra<BkHeader, TxBody>
where
    BkHeader: Clone,
    TxBody: Clone,
{
    pub body: Vec<Tx<TxBody>>,
    pub header: BkHeader,
    #[serde(rename = "headerHash")]
    pub header_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Certificate {
    #[serde(rename = "stakeKeyRegistration")]
    StakeKeyRegistration(String),
    #[serde(rename = "poolRegistration")]
    PoolRegistration {
        id: String,
        vrf: String,
        pledge: u64,
        cost: u64,
        margin: String,
        #[serde(rename = "rewardAccount")]
        reward_account: String,
        owners: Vec<String>,
        relays: Vec<Relay>,
        metadata: Option<PoolMetaData>,
    },
    #[serde(rename = "stakeDelegation")]
    StakeDelegation {
        delegator: String,
        delegatee: String,
    },
    #[serde(rename = "poolRetirement")]
    PoolRetirement {
        #[serde(rename = "poolId")]
        pool_id: String,
        #[serde(rename = "retirementEpoch")]
        retirement_epoch: u64,
    },
    #[serde(rename = "moveInstantaneousRewards")]
    MoveInstantaneousRewards {
        pot: String,
        rewards: HashMap<String, u64>,
    },
    #[serde(rename = "stakeKeyDeregistration")]
    StakeKeyDeregistration(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxBodyAllegra {
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub certificates: Vec<Certificate>,
    pub withdrawals: HashMap<String, u64>,
    pub fee: u64,
    #[serde(rename = "validityInterval")]
    pub validity_interval: ValidityInterval,
    pub update: Option<Update>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxBodyMary {
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub certificates: Vec<Certificate>,
    pub withdrawals: HashMap<String, u64>,
    pub fee: u64,
    #[serde(rename = "validityInterval")]
    pub validity_interval: ValidityInterval,
    pub update: Option<Update>,
    pub mint: Mint,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxBodyShelley {
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub certificates: Vec<Certificate>,
    pub withdrawals: HashMap<String, u64>,
    pub fee: u64,
    #[serde(rename = "timeToLive")]
    pub ttl: u64,
    pub update: Option<Update>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxMetadata {
    pub hash: String,
    pub body: BlobMetadata,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlobMetadata {
    pub blob: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PoolMetaData {
    pub url: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Relay {
    Ip {
        port: u16,
        ipv4: Option<String>,
        ipv6: Option<String>,
    },
    Hostname {
        hostname: String,
        port: Option<u16>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mint {
    pub coins: u64,
    pub assets: HashMap<String, i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Update {
    pub proposal: HashMap<String, UpdateProposal>,
    pub epoch: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProposal {
    #[serde(rename = "minFeeCoefficient")]
    pub min_fee_coefficient: Option<u64>,
    #[serde(rename = "minFeeConstant")]
    pub min_fee_constant: Option<u64>,
    #[serde(rename = "maxBlockBodySize")]
    pub max_block_body_size: Option<u64>,
    #[serde(rename = "maxBlockHeaderSize")]
    pub max_block_header_size: Option<u64>,
    #[serde(rename = "maxTxSize")]
    pub max_tx_size: Option<u64>,
    #[serde(rename = "stakeKeyDeposit")]
    pub stake_key_deposit: Option<u64>,
    #[serde(rename = "poolDeposit")]
    pub pool_deposit: Option<u64>,
    #[serde(rename = "poolRetirementEpochBound")]
    pub pool_retirement_epoch_bound: Option<u64>,
    #[serde(rename = "desiredNumberOfPools")]
    pub desired_number_of_pool: Option<u64>,
    #[serde(rename = "poolInfluence")]
    pub pool_influence: Option<String>,
    #[serde(rename = "monetaryExpansion")]
    pub monetary_expansion: Option<String>,
    #[serde(rename = "treasuryExpansion")]
    pub treasury_expansion: Option<String>,
    #[serde(rename = "decentralizationParameter")]
    pub decentralization_parameter: Option<String>,
    #[serde(rename = "extraEntropy")]
    pub extra_entropy: Option<String>,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: Option<ProtocolVersion>,
    #[serde(rename = "minUtxoValue")]
    pub min_utxo_value: Option<u64>,
    #[serde(rename = "minPoolCost")]
    pub min_pool_cost: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValidityInterval {
    #[serde(rename = "invalidBefore")]
    pub invalid_before: Option<u64>,
    #[serde(rename = "invalidHereafter")]
    pub invalid_hereafter: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShelleyHeader {
    #[serde(rename = "blockHeight")]
    pub block_height: u64,
    pub slot: u64,
    #[serde(rename = "prevHash")]
    pub prev_hash: String,
    #[serde(rename = "issuerVk")]
    pub issuer_vk: String,
    #[serde(rename = "blockSize")]
    pub block_size: u64,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
}
