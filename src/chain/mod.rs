use std::{
    collections::HashMap,
    io::{Read, Write},
};

use color_eyre::eyre::{eyre, Result};
use tracing::{info, warn};

use std::fs::File;

use crate::data::{Block, Point, PointOrOrigin, RResult, Tip};
pub struct Chain {
    data: HashMap<u64, Chunk>,
    tip: Option<Tip>,
    current_epoch: u64,
}

pub enum ChainEvent {
    Collection(Vec<Block>),
    RevertFork(Vec<Block>),
}

pub struct Chunk {
    data: Option<Vec<Block>>,
    epoch: u64,
}

impl Chunk {
    pub fn new(epoch: u64) -> Self {
        Self {
            data: Some(Vec::new()),
            epoch,
        }
    }

    pub fn in_memory(&self) -> bool {
        self.data.is_some()
    }

    pub fn dump(&mut self) -> Result<()> {
        let mut file = File::create(format!("data/{}.bin", self.epoch))?;

        let bin = bincode::serialize(&self.data).unwrap();

        file.write_all(&bin)?;
        self.data = None;

        Ok(())
    }

    pub fn load(&mut self) -> Result<()> {
        let mut file = File::open(format!("data/{}.bin", self.epoch))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf);

        self.data = Some(bincode::deserialize(&buf)?);
        Ok(())
    }
}

impl Chain {
    pub fn new(buffer_capacity: usize) -> Self {
        Self {
            data: HashMap::new(),
            tip: None,
            current_epoch: 0,
        }
    }

    pub fn add(&mut self, action: RResult) -> Option<ChainEvent> {
        // println!("{:#?}", &action);
        match action {
            RResult::RollForward { block, tip } => {
                self.tip = Some(tip);
                let epoch = block.epoch();

                if !self.data.contains_key(&block.epoch()) {
                    self.data.insert(epoch, Chunk::new(epoch));
                    self.current_epoch = epoch;
                    if self.current_epoch != 0 {
                        self.data.get_mut(&(self.current_epoch - 1)).unwrap().dump();
                    }
                }

                match self.data.get_mut(&epoch) {
                    Some(chunk) => chunk.data.as_mut().unwrap().push(block),
                    None => (),
                }
                // if let Some(c) = self.collect() {
                //     Some(ChainEvent::Collection(c))
                // } else {
                //     None
                // }
                None
            }
            RResult::RollBackward { point, tip } => {
                let epoch = point.epoch();
                let idx = if !self.data.is_empty() {
                    self.data
                        .get(&epoch)
                        .unwrap()
                        .data
                        .as_ref()
                        .unwrap()
                        .iter()
                        .position(|block| {
                            if let PointOrOrigin::Point(Point { hash, .. }) = &point {
                                block.hash() == *hash
                            } else {
                                false
                            }
                        })
                } else {
                    None
                };

                self.tip = Some(tip);

                match idx {
                    Some(i) => {
                        warn!(
                            "Reverting block: {} / {}",
                            i,
                            self.data.get(&epoch).unwrap().data.as_ref().unwrap().len()
                        );
                        let d = self
                            .data
                            .get_mut(&epoch)
                            .unwrap()
                            .data
                            .as_mut()
                            .unwrap()
                            .drain(i..);
                        Some(ChainEvent::RevertFork(d.collect()))
                    }
                    None => None,
                }
            }
            _ => unimplemented!(),
        }
    }

    // fn collect(&mut self) -> Option<Vec<Block>> {
    //     if self.buffer.len() == 2 * self.buffer_capacity {
    //         let d = self.buffer.drain(..self.buffer_capacity);
    //         Some(d.collect())
    //     } else {
    //         None
    //     }
    // }

    pub fn sync(&self) -> Option<f32> {
        match (
            &self
                .data
                .get(&self.current_epoch)
                .map(|c| c.data.as_ref().unwrap().last().unwrap()),
            &self.tip,
        ) {
            (Some(block), Some(tip)) => {
                if block.hash() == tip.hash {
                    // info!("{} {}", block.slot(), block.hash());
                    // info!("{} {}", tip.slot, tip.hash);
                    // info!("Block time: [{}] {:#?}", block.epoch(), block.timestamp());
                    None
                } else {
                    // info!("{} {}", block.slot(), block.hash());
                    // info!("{} {}", tip.slot, tip.hash);
                    // info!("Block time: [{}] {:#?}", block.epoch(), block.timestamp());
                    // println!("{:#?}", (self.buffer.last(), &self.tip));

                    Some(block.slot() as f32 / tip.slot as f32 * 100.0)
                }
            }
            _ => Some(0.0),
        }
    }
}
