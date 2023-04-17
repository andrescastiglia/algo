use super::{merge::merge, providers::Providers};
use anyhow::Result;
use common::{
    orderbook::{orderbook_aggregator_server::OrderbookAggregator, Empty, Summary},
    ConfigRef,
};
use log::info;
use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};

pub struct Orderbook {
    config: ConfigRef,
    providers: Providers,
}

impl Orderbook {
    pub fn new(config: ConfigRef) -> Self {
        info!("initialize {}", config.pair());

        let providers = Providers::new(Arc::clone(&config));

        Self { config, providers }
    }

    pub fn connect(&self) -> Result<()> {
        self.providers.connect()
    }
}

impl Drop for Orderbook {
    fn drop(&mut self) {
        self.providers.disconnect().ok();
    }
}

#[async_trait]
impl OrderbookAggregator for Orderbook {
    async fn book_summary(&self, _request: Request<Empty>) -> Result<Response<Summary>, Status> {
        match self.providers.retrieve().await {
            Ok(summaries) => {
                let summary = merge(Arc::clone(&self.config), summaries);
                Ok(Response::new(summary))
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
