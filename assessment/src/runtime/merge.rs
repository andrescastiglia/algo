use std::ops::Sub;

use common::{orderbook::Summary, ConfigRef};

pub fn merge(config: ConfigRef, summaries: Vec<Summary>) -> Summary {
    let mut summary = Summary::default();

    for s in summaries {
        summary.asks.extend(s.asks);
        summary.bids.extend(s.bids);
    }

    summary.asks.sort_by(|x, y| x.price.total_cmp(&y.price));
    summary.asks.truncate(config.top());

    summary.bids.sort_by(|x, y| y.price.total_cmp(&x.price));
    summary.bids.truncate(config.top());

    summary.spread = match (summary.asks.last(), summary.bids.first()) {
        (Some(ask), Some(bid)) => ask.price.sub(bid.price),
        _ => 0.0,
    };

    summary
}
