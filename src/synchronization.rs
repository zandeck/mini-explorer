use std::sync::Arc;

use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tokio_tungstenite::tungstenite::http::Uri;

use crate::chain::{Chain, ChainEvent};
use crate::data::{PointOrOrigin, RResult};
use crate::ws::{self, Connection};

#[derive(Debug)]
pub struct Engine {
    pub uri: Uri,
    pub chain: Arc<Mutex<Chain>>,
    pub connection: Arc<Connection>,
}

impl Engine {
    pub fn new(uri: Uri) -> (Box<Self>, ReceiverStream<ChainEvent>) {
        let chain = Arc::new(Mutex::new(Chain::new(2000)));
        let (tx, rx) = mpsc::channel(2000);
        let (tx_engine, rx_engine) = mpsc::channel(2000);
        let points = vec![PointOrOrigin::origin()];
        let connection = ws::Connection::new(uri.clone(), points, tx);

        let cloned_chain = chain.clone();
        tokio::spawn(async move {
            let mut rs = ReceiverStream::new(rx);
            while let Some(r) = rs.next().await {
                match r {
                    RResult::RollBackward { .. } | RResult::RollForward { .. } => {
                        let mut c = cloned_chain.lock().await;
                        let v = c.add(r);
                        // dbg!(&v);
                        let _ = tx_engine.send(v).await;
                    }
                    _ => (),
                }
            }
        });

        (
            Box::new(Self {
                uri,
                connection: Arc::new(connection),
                chain,
            }),
            ReceiverStream::new(rx_engine),
        )
    }

    pub async fn start(&self) {
        let connection = self.connection.clone();
        tokio::spawn(async move {
            let _ = connection.run().await;
        });
    }
}
