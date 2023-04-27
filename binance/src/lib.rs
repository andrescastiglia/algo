use anyhow::{Context, Result};
use common::{
    orderbook::{Level, Summary},
    ConfigRef, Provider,
};
use log::info;
use parking_lot::RwLock;
use serde::Deserialize;
use std::net::TcpStream;
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};

#[derive(Deserialize)]
struct Depth {
    #[serde(rename = "lastUpdateId")]
    _last_update_id: u64,
    bids: Vec<[String; 2]>,
    asks: Vec<[String; 2]>,
}

pub struct Binance {
    _config: ConfigRef,
    socket: RwLock<WebSocket<MaybeTlsStream<TcpStream>>>,
}

impl Drop for Binance {
    fn drop(&mut self) {
        info!("binance disconnect");
        self.socket.write().close(None).ok();
    }
}

impl Provider for Binance {
    fn name(&self) -> &'static str {
        "Binance"
    }

    fn subscribe(&self) -> Result<()> {
        Ok(())
    }

    fn unsubscribe(&self) -> Result<()> {
        Ok(())
    }

    fn summary(&self) -> Result<Summary> {
        let message = self.read()?;
        let depth: Depth = serde_json::from_str(message.to_text()?)?;

        let mut summary = Summary::default();

        for order in depth.asks {
            let level = Level {
                exchange: String::from(self.name()),
                price: order[0].parse()?,
                amount: order[1].parse()?,
            };
            summary.asks.push(level)
        }

        for order in depth.bids {
            let level = Level {
                exchange: String::from(self.name()),
                price: order[0].parse()?,
                amount: order[1].parse()?,
            };
            summary.bids.push(level)
        }

        Ok(summary)
    }
}

impl Binance {
    pub fn new(config: ConfigRef) -> Self {
        let url = config.binance_url();

        let depth = format!("{}@depth20@100ms", config.pair());
        let url = url.join(depth.as_str()).expect("failed to build url");

        info!("binance connect {}", url);

        let (socket, _) = connect(url).expect("failed to connect to binance");

        Self {
            _config: config,
            socket: RwLock::new(socket),
        }
    }

    fn read(&self) -> Result<Message> {
        self.socket
            .write()
            .read_message()
            .with_context(|| "Failed to read")
    }
}
