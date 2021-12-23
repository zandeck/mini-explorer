use std::{collections::HashMap, io::Write};

use color_eyre::eyre::{eyre, Result};
use std::fs::File;
use tracing::{info, warn};

use crate::data::{Block, Point, PointOrOrigin, RResult, Tip};

#[derive(Debug)]
pub struct Chain {
    data: HashMap<u64, Chunk>,
    pub tip: Option<Tip>,
    pub current_epoch: u64,
}

#[derive(Debug)]
pub enum SyncProgress<T> {
    Synchronizing(T, Block, Tip),
    Synchronized(Tip),
    Unsynchronized,
}

#[derive(Debug)]
pub enum ChainEvent {
    Collection(Vec<Block>),
    Synchronizing(SyncProgress<f32>),
    RevertFork(Vec<Block>),
}

#[derive(Debug)]
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

    // pub fn in_memory(&self) -> bool {
    //     self.data.is_some()
    // }

    pub fn dump(&mut self) -> Result<()> {
        let mut file = File::create(format!("data/{}.bin", self.epoch))?;

        let bin = bincode::serialize(&self.data).unwrap();

        file.write_all(&bin)?;
        self.data = None;

        Ok(())
    }

    // pub fn load(&mut self) -> Result<()> {
    //     let mut file = File::open(format!("data/{}.bin", self.epoch))?;
    //     let mut buf = Vec::new();
    //     file.read_to_end(&mut buf);

    //     self.data = Some(bincode::deserialize(&buf)?);
    //     Ok(())
    // }
}

impl Chain {
    pub fn new(_buffer_capacity: usize) -> Self {
        Self {
            data: HashMap::new(),
            tip: None,
            current_epoch: 0,
        }
    }

    pub fn add(&mut self, action: RResult) -> ChainEvent {
        // info!("{:?}", &action);
        match action {
            RResult::RollForward { block, tip } => {
                self.tip = Some(tip);
                let epoch = block.epoch();
                // dbg!(block.epoch());
                // dbg!(!self.data.contains_key(&block.epoch()));
                if !self.data.contains_key(&block.epoch()) {
                    self.data.insert(epoch, Chunk::new(epoch));
                    self.current_epoch = epoch;
                    if self.current_epoch != 0 {
                        self.data.get_mut(&(self.current_epoch - 1)).unwrap().dump();
                    }
                }
                // dbg!(self.data.get_mut(&epoch));
                match self.data.get_mut(&epoch) {
                    Some(chunk) => chunk.data.as_mut().unwrap().push(block),
                    None => (),
                }

                // if let Some(c) = self.collect() {
                //     Some(ChainEvent::Collection(c))
                // } else {
                //     None
                // }
                ChainEvent::Synchronizing(self.sync())
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
                        ChainEvent::RevertFork(d.collect())
                    }
                    None => ChainEvent::Synchronizing(self.sync()),
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

    pub fn sync(&self) -> SyncProgress<f32> {
        // dbg!(&self.data.get(&self.current_epoch));
        match (
            self.data
                .get(&self.current_epoch)
                .map(|c| c.data.as_ref().unwrap().last().unwrap()),
            &self.tip,
        ) {
            (Some(ref block), Some(ref tip)) => {
                if block.hash() == tip.hash {
                    // info!("{} {}", block.slot(), block.hash());
                    // info!("{} {}", tip.slot, tip.hash);
                    // info!("Block time: [{}] {:#?}", block.epoch(), block.timestamp());
                    SyncProgress::Synchronized(tip.clone())
                } else {
                    // info!("{} {}", block.slot(), block.hash());
                    // info!("{} {}", tip.slot, tip.hash);
                    // info!("Block time: [{}] {:#?}", block.epoch(), block.timestamp());
                    // println!("{:#?}", (self.buffer.last(), &self.tip));

                    SyncProgress::Synchronizing(
                        block.slot() as f32 / tip.slot as f32 * 100.0,
                        (**block).clone(),
                        tip.clone(),
                    )
                }
            }
            _ => SyncProgress::Unsynchronized,
        }
    }
}

// impl<H, E> iced_native::subscription::Recipe<H, E> for Chain
// where
//     H: Hasher,
// {
//     type Output = Progress;

//     fn hash(&self, state: &mut H) {
//         struct Marker;
//         std::any::TypeId::of::<Marker>().hash(state);

//         self.uri.hash(state);
//     }

//     fn stream(
//         self: Box<Self>,
//         _input: futures::stream::BoxStream<'static, Uri>,
//     ) -> futures::stream::BoxStream<'static, Self::Output> {
//         let id = self.uri;

//         Box::pin(futures::stream::unfold(
//             State::Ready(self.uri.to_string()),
//             move |state| async move {
//                 match state {
//                     State::Ready(url) => {
//                         let response = reqwest::get(&url).await;

//                         match response {
//                             Ok(response) => {
//                                 if let Some(total) = response.content_length() {
//                                     Some((
//                                         (id, Progress::Started),
//                                         State::Downloading {
//                                             response,
//                                             total,
//                                             downloaded: 0,
//                                         },
//                                     ))
//                                 } else {
//                                     Some(((id, Progress::Errored), State::Finished))
//                                 }
//                             }
//                             Err(_) => Some(((id, Progress::Errored), State::Finished)),
//                         }
//                     }
//                     State::Downloading {
//                         mut response,
//                         total,
//                         downloaded,
//                     } => match response.chunk().await {
//                         Ok(Some(chunk)) => {
//                             let downloaded = downloaded + chunk.len() as u64;

//                             let percentage = (downloaded as f32 / total as f32) * 100.0;

//                             Some((
//                                 (id, Progress::Advanced(percentage)),
//                                 State::Downloading {
//                                     response,
//                                     total,
//                                     downloaded,
//                                 },
//                             ))
//                         }
//                         Ok(None) => Some(((id, Progress::Finished), State::Finished)),
//                         Err(_) => Some(((id, Progress::Errored), State::Finished)),
//                     },
//                     State::Finished => {
//                         // We do not let the stream die, as it would start a
//                         // new download repeatedly if the user is not careful
//                         // in case of errors.
//                         let _: () = iced::futures::future::pending().await;

//                         None
//                     }
//                 }
//             },
//         ))
//     }
// }

#[derive(Debug, Clone)]
pub enum Progress {
    Started,
    Advanced(f32),
    Finished,
    Errored,
}

// pub enum State {
//     Ready(String),
//     Downloading {
//         response: reqwest::Response,
//         total: u64,
//         downloaded: u64,
//     },
//     Finished,
// }
