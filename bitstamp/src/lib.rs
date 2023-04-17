use anyhow::{anyhow, Context, Result};
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
struct Response {
    data: OrderBook,
    #[serde(rename = "channel")]
    _channel: String,
    #[serde(rename = "event")]
    _event: String,
}

#[derive(Deserialize)]
struct OrderBook {
    #[serde(rename = "timestamp")]
    _timestamp: String,
    #[serde(rename = "microtimestamp")]
    _microtimestamp: String,
    bids: Vec<[String; 2]>,
    asks: Vec<[String; 2]>,
}

pub struct Bitstamp {
    config: ConfigRef,
    socket: RwLock<WebSocket<MaybeTlsStream<TcpStream>>>,
}

impl Drop for Bitstamp {
    fn drop(&mut self) {
        info!("bitstamp disconnect");
        self.socket.write().close(None).ok();
    }
}

impl Provider for Bitstamp {
    fn name(&self) -> &str {
        "Bitstamp"
    }

    fn subscribe(&self) -> Result<()> {
        let subscribe = format!(
            "{{\"event\":\"bts:subscribe\",\"data\":{{\"channel\":\"order_book_{}\"}}}}",
            self.config.pair()
        );
        info!("bitstamp subscribe - {}", subscribe.as_str());

        let request = Message::Text(subscribe);
        self.write(request)?;

        let response = self.read()?;
        info!("bitstamp subscribe - {}", response.to_text()?);

        Ok(())
    }

    fn unsubscribe(&self) -> Result<()> {
        let unsubscribe = format!(
            "{{\"event\":\"bts:unsubscribe\",\"data\":{{\"channel\":\"order_book_{}\"}}}}",
            self.config.pair()
        );
        info!("bitstamp unsubscribe - {}", unsubscribe.as_str());

        let request = Message::Text(unsubscribe);
        self.write(request)?;

        let response = self.read()?;
        info!("bitstamp unsubscribe - {}", response.to_text()?);

        Ok(())
    }

    fn summary(&self) -> Result<Summary> {
        match self.read()? {
            Message::Text(message) => {
                let response: Response = serde_json::from_str(message.as_str())?;
                let orderbook = response.data;

                let mut summary = Summary::default();

                for order in orderbook.asks {
                    let level = Level {
                        exchange: String::from(self.name()),
                        price: order[0].parse()?,
                        amount: order[1].parse()?,
                    };
                    summary.asks.push(level);
                }

                for order in orderbook.bids {
                    let level = Level {
                        exchange: String::from(self.name()),
                        price: order[0].parse()?,
                        amount: order[1].parse()?,
                    };
                    summary.bids.push(level);
                }

                Ok(summary)
            }
            _ => Err(anyhow!("unexpected")),
        }
    }
}

impl Bitstamp {
    pub fn new(config: ConfigRef) -> Self {
        let url = config.bitstamp_url();
        info!("bitstamp connect - {}", url);

        let (socket, _) = connect(url).expect("failed to connect to bitstamp");

        Self {
            config,
            socket: RwLock::new(socket),
        }
    }

    fn write(&self, request: Message) -> Result<()> {
        self.socket
            .write()
            .write_message(request)
            .with_context(|| "Failed to write")
    }

    fn read(&self) -> Result<Message> {
        self.socket
            .write()
            .read_message()
            .with_context(|| "Failed to read")
    }
}
