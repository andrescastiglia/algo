use serde::Deserialize;
use std::slice::Iter;

#[derive(Deserialize)]
pub struct OrderBook {
    #[serde(rename = "timestamp")]
    _timestamp: String,
    #[serde(rename = "microtimestamp")]
    _microtimestamp: String,
    bids: Vec<[String; 2]>,
    asks: Vec<[String; 2]>,
}

impl OrderBook {
    pub fn asks(&self) -> Iter<[String; 2]> {
        self.asks.iter()
    }
    pub fn bids(&self) -> Iter<[String; 2]> {
        self.bids.iter()
    }
}
