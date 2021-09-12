use tracing::{info, warn};

use crate::data::{Block, Point, PointOrOrigin, RResult, Tip};

pub struct Chain {
    buffer: Vec<Block>,
    buffer_capacity: usize,
    tip: Option<Tip>,
}

pub enum ChainEvent {
    Collection(Vec<Block>),
    RevertFork(Vec<Block>),
}

impl Chain {
    pub fn new(buffer_capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(2 * buffer_capacity),
            buffer_capacity,
            tip: None,
        }
    }

    pub fn recent(&self) -> &Vec<Block> {
        &self.buffer
    }

    pub fn add(&mut self, action: RResult) -> Option<Vec<Block>> {
        match action {
            RResult::RollForward { block, tip } => {
                self.buffer.push(block);
                self.tip = Some(tip);
            }
            RResult::RollBackward { point, tip } => {
                let idx = self.buffer.iter().position(|block| {
                    if let PointOrOrigin::Point(Point { hash, .. }) = &point {
                        block.hash() == *hash
                    } else {
                        false
                    }
                });
                match idx {
                    Some(i) => {
                        warn!("{}", i);
                        unimplemented!()
                    }
                    None => (),
                };
                self.tip = Some(tip);
            }
            _ => unimplemented!(),
        }
        self.collect()
    }

    fn collect(&mut self) -> Option<Vec<Block>> {
        if self.buffer.len() == 2 * self.buffer_capacity {
            let d = self.buffer.drain(..self.buffer_capacity);
            Some(d.collect())
        } else {
            None
        }
    }

    pub fn sync(&self) -> Option<f32> {
        match (self.buffer.last(), &self.tip) {
            (Some(block), Some(tip)) => {
                if block.hash() == tip.hash {
                    None
                } else {
                    Some(block.slot() as f32 / tip.slot as f32 * 100.0)
                }
            }
            _ => Some(0.0),
        }
    }
}
