use anyhow::Result;
use assessment::{cli::show::show_once, runtime::orderbook::Orderbook};
use common::{config::Config, orderbook::orderbook_aggregator_server::OrderbookAggregatorServer};
use std::sync::Arc;
use tonic::transport::Server;

#[tokio::main]
pub async fn main() -> Result<()> {
    env_logger::init();

    let config = Config::as_ref();

    if config.cli() {
        show_once().await?;
    } else {
        let orderbook = Orderbook::new(Arc::clone(&config));
        orderbook.connect()?;

        Server::builder()
            .add_service(OrderbookAggregatorServer::new(orderbook))
            .serve(config.local_bind())
            .await?;
    }
    Ok(())
}
