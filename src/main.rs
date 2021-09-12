use color_eyre::eyre::Result;
use structopt::StructOpt;
use tokio::sync::mpsc;

mod chain;
mod cli;
mod data;
mod storage;
mod ws;

use data::{PointOrOrigin, RResult};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    let opt = init();
    let mut chain = chain::Chain::new(2000);
    // let mut storage = storage::Mongodb::new(&opt);

    let (tx, mut rx) = mpsc::channel(32);

    let connection = ws::Connection::new(&opt, tx);

    tokio::spawn(async move {
        let _ = connection.run().await;
    });

    while let Some(message) = rx.recv().await {
        // println!("GOT = {:?}", message);
        match &message {
            RResult::RollBackward { .. } | RResult::RollForward { .. } => {
                let c = chain.add(message);
                if c.is_some() {
                    info!("Supposed to collect something here");
                }
                // if let Some(potential_storage) = chain.add(message) {
                //     storage.insert_many(potential_storage).await?;
                // }
                if let Some(progress) = chain.sync() {
                    warn!("Chain is not synced: {:.2}", progress);
                } else {
                    info!("Chain is synced");
                }
            }
            _ => (),
        }
    }

    Ok(())
}

fn init() -> cli::CLI {
    color_eyre::install();
    dotenv::dotenv().unwrap();
    // let file_appender = tracing_appender::rolling::hourly("./", "prefix.log");
    // let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    // tracing_subscriber::fmt().with_writer(non_blocking).init();
    tracing_subscriber::fmt().init();
    let opt = cli::CLI::from_args();
    opt
}
