use serde::{Deserialize, Serialize};

pub mod alonzo;
pub mod byron;
pub mod shelley;

use alonzo::TxBodyAlonzo;
use byron::{ByronBlockEra, ByronHeader, TxBodyByron};
use shelley::{
    ShelleyBlockEra, ShelleyHeader, TxBodyAllegra, TxBodyMary, TxBodyShelley, TxMetadata,
};

#[derive(Serialize, Deserialize, Debug)]
pub enum Block {
    #[serde(rename = "byron")]
    Byron(ByronBlockEra<ByronHeader, TxBodyByron>),
    #[serde(rename = "shelley")]
    Shelley(ShelleyBlockEra<ShelleyHeader, TxBodyShelley>),
    #[serde(rename = "allegra")]
    Allegra(ShelleyBlockEra<ShelleyHeader, TxBodyAllegra>),
    #[serde(rename = "mary")]
    Mary(ShelleyBlockEra<ShelleyHeader, TxBodyMary>),
    #[serde(rename = "alonzo")]
    Alonzo(ShelleyBlockEra<ShelleyHeader, TxBodyAlonzo>),
}

impl Block {
    pub fn slot(&self) -> u64 {
        match self {
            Self::Shelley(block) => block.header.slot,
            Self::Allegra(block) => block.header.slot,
            Self::Mary(block) => block.header.slot,
            Self::Alonzo(block) => block.header.slot,
            _ => 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tx<Body> {
    pub id: String,
    pub body: Body,
    pub metadata: Option<TxMetadata>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxIn {
    #[serde(rename = "txId")]
    pub tx_id: String,
    pub index: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxOut {
    pub address: String,
    pub value: Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Value {
    pub coins: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tip {
    pub slot: u64,
    pub hash: String,
    #[serde(rename = "blockNo")]
    pub block_no: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProtocolVersion {
    pub minor: u64,
    pub major: u64,
    pub patch: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    #[serde(rename = "type")]
    pub ttype: String,
    pub version: String,
    #[serde(rename = "servicename")]
    pub service_name: String,
    #[serde(rename = "methodname")]
    pub method: String,
    pub args: Option<ArgsInner>,
}

impl Request {
    pub fn new(args: Args) -> Self {
        match args {
            Args::FindIntersect(v) => Request {
                ttype: "jsonwsp/request".into(),
                version: "1.0".into(),
                service_name: "ogmios".into(),
                method: "FindIntersect".into(),
                args: Some(ArgsInner::FindIntersect(v)),
            },
            Args::RequestNext => Request {
                ttype: "jsonwsp/request".into(),
                version: "1.0".into(),
                service_name: "ogmios".into(),
                method: "RequestNext".into(),
                args: None,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    #[serde(rename = "type")]
    pub ttype: String,
    pub version: String,
    #[serde(rename = "servicename")]
    pub service_name: String,
    #[serde(rename = "methodname")]
    pub method: String,
    pub result: RResult,
    pub reflection: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RResult {
    IntersectionFound { point: PointOrOrigin, tip: Tip },
    RollBackward { point: PointOrOrigin, tip: Tip },
    RollForward { block: Block, tip: Tip },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Point {
    pub slot: u64,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PointOrOrigin {
    Point(Point),
    #[serde(rename = "origin")]
    Origin(String),
}

impl PointOrOrigin {
    pub fn point(slot: u64, hash: String) -> Self {
        Self::Point(Point { slot, hash })
    }

    pub fn origin() -> Self {
        Self::Origin("origin".to_string())
    }
}

#[derive(Debug)]
pub enum Args {
    FindIntersect(Vec<PointOrOrigin>),
    RequestNext,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ArgsInner {
    #[serde(rename = "points")]
    FindIntersect(Vec<PointOrOrigin>),
}
