use mongodb::{options::ClientOptions, Client, Database};
use color_eyre::eyre::{Result, eyre};
use crate::data::{Args, PointOrOrigin, RResult, Request, Response, Block};
use crate::cli::CLI;

#[derive(Debug, Default)]
pub struct Mongodb {
    pub address: String,
    pub db_name: String,
    pub client: Option<Client>,
    pub db: Option<Database>,
}

impl Mongodb {
    pub fn new(opt: &CLI) -> Self {
        Self { address: opt.mongodb.clone().unwrap(), db_name: opt.db_name.clone().unwrap(), ..Self::default() }
    }

    pub async fn connect(&mut self) -> Result<()> {
        let client_options = ClientOptions::parse(self.address.clone()).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(&self.db_name);
        self.client = Some(client);
        self.db = Some(db);
        Ok(())
    }

    pub async fn insert_many(&mut self, results: Vec<Block> ) -> Result<()> {
        match &self.db {
            Some(db) => {
                let collection = db.collection::<Block>("chain");
                collection.insert_many(results, None).await?;
                Ok(())
            },
            None => Err(eyre!("Problem inserting in the db."))
        }
    }
}

