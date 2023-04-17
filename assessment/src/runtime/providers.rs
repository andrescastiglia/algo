use anyhow::Result;
use common::{orderbook::Summary, ConfigRef, Provider};
use log::info;
use std::sync::Arc;
use tokio::task::{self, JoinHandle};

#[cfg(feature = "binance")]
use binance::Binance;
#[cfg(feature = "bitstamp")]
use bitstamp::Bitstamp;

type ProviderRef = Arc<Box<dyn Provider>>;

pub struct Providers {
    collection: Vec<ProviderRef>,
}

impl Providers {
    pub fn new(config: ConfigRef) -> Self {
        let providers: Vec<ProviderRef> = vec![
            #[cfg(feature = "binance")]
            Arc::new(Box::new(Binance::new(Arc::clone(&config)))),
            #[cfg(feature = "bitstamp")]
            Arc::new(Box::new(Bitstamp::new(Arc::clone(&config)))),
        ];

        Self {
            collection: providers,
        }
    }

    pub fn connect(&self) -> Result<()> {
        info!("connect");

        for i in 0..self.collection.len() {
            self.collection[i].subscribe()?;
        }

        Ok(())
    }

    pub async fn retrieve(&self) -> Result<Vec<Summary>> {
        info!("retrieve");

        let count = self.collection.len();

        let mut handles = Vec::<JoinHandle<Result<Summary>>>::with_capacity(count);

        for i in 0..self.collection.len() {
            let provider = Arc::clone(&self.collection[i]);
            handles.push(task::spawn_blocking(move || provider.summary()));
        }

        let mut summaries = Vec::<Summary>::with_capacity(count);

        for handle in handles {
            summaries.push(handle.await??);
        }

        Ok(summaries)
    }

    pub fn disconnect(&self) -> Result<()> {
        info!("disconnect");

        for i in 0..self.collection.len() {
            self.collection[i].unsubscribe()?;
        }

        Ok(())
    }
}
