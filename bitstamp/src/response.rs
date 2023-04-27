use crate::orderbook::OrderBook;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Response {
    data: OrderBook,
    #[serde(rename = "channel")]
    _channel: String,

    #[serde(rename = "event")]
    _event: String,
}

impl Response {
    pub fn orderbook(&self) -> &OrderBook {
        &self.data
    }
}
