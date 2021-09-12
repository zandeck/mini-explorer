use color_eyre::eyre::Result;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{
    connect_async,
    tungstenite::http::{self, Uri},
    tungstenite::Message,
};

use crate::data::{Args, PointOrOrigin, RResult, Request, Response};
use tokio::sync::mpsc::Sender;
use tracing::info;

#[derive(Debug)]
pub struct Connection {
    ws: Uri,
    points: Vec<PointOrOrigin>,
    channel: Sender<RResult>,
}

impl Connection {
    pub fn new(ws: Uri, points: Vec<PointOrOrigin>, channel: Sender<RResult>) -> Self {
        let mut points = points.clone();
        points.push(PointOrOrigin::origin());

        Connection {
            ws,
            points,
            channel,
        }
    }

    fn query(&self, args: Args) -> Message {
        let q = Request::new(args);

        Message::Text(serde_json::to_string(&q).unwrap())
    }

    pub async fn run(&self) -> Result<()> {
        let req = http::Request::builder()
            .uri(self.ws.clone())
            .header("Sec-WebSocket-Protocol", "ogmios.v1:compact")
            .body(())
            .unwrap();

        let (ws_stream, _) = connect_async(req).await.expect("Failed to connect");

        let (mut write, mut read) = ws_stream.split();

        let msg = self.query(Args::FindIntersect(self.points.clone()));
        write.send(msg).await?;

        for _ in 1..1000 {
            let msg = self.query(Args::RequestNext);
            write.send(msg).await?;
            info!("Send initial request");
        }

        loop {
            let message = read.next().await;
            match message {
                Some(Ok(Message::Text(t))) => {
                    let resp: Response = serde_json::from_str(&t)?;

                    self.channel.send(resp.result).await?;

                    let msg = self.query(Args::RequestNext);
                    write.send(msg).await?;
                }
                _ => println!("{:?}", message),
            }
        }
    }
}
