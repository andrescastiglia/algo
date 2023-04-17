use anyhow::Result;
use common::orderbook::{orderbook_aggregator_client::OrderbookAggregatorClient, Empty, Summary};
use tonic::{transport::Channel, Request};

pub async fn show_once() -> Result<()> {
    let channel = Channel::from_static("http://[::1]:50051").connect().await?;

    let mut client = OrderbookAggregatorClient::new(channel);

    let request = Request::new(Empty::default());

    let summary: Summary = client.book_summary(request).await?.into_inner();

    println!("amount\tprice");
    for ask in summary.asks {
        println!("{:.6}\t{:.6}", ask.amount, ask.price);
    }

    println!("spread: {:.6}", summary.spread);

    for bid in summary.bids {
        println!("{:.6}\t{:.6}", bid.amount, bid.price);
    }

    Ok(())
}
