use structopt::StructOpt;
use tokio_tungstenite::tungstenite::http::Uri;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Mini-Explorer",
    about = "Tool to connect to an Ogmios server to process data."
)]
pub struct CLI {
    #[structopt(short, long)]
    pub ws: Uri,
    #[structopt(short, long)]
    pub block: String,
    #[structopt(short, long)]
    pub slot: u64,
}
