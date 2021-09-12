use color_eyre::eyre::Result;
use structopt::StructOpt;
use tokio::sync::mpsc;

mod cli;
mod data;
mod ws;

use data::PointOrOrigin;

#[tokio::main]
async fn main() -> Result<()> {
    let opt = init();

    let args = vec![
        PointOrOrigin::point(opt.slot, opt.block),
        PointOrOrigin::origin(),
    ];

    let (tx, mut rx) = mpsc::channel(32);

    let connection = ws::Connection::new(opt.ws, args, tx);

    tokio::spawn(async move {
        let _ = connection.run().await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {:?}", message);
    }

    Ok(())
}

fn init() -> cli::CLI {
    dotenv::dotenv().unwrap();
    // let file_appender = tracing_appender::rolling::hourly("./", "prefix.log");
    // let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    // tracing_subscriber::fmt().with_writer(non_blocking).init();
    tracing_subscriber::fmt().init();
    let opt = cli::CLI::from_args();
    opt
}
