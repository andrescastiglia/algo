use crate::response::Response;
use anyhow::{anyhow, Result};
use common::{
    orderbook::{Level, Summary},
    ConfigRef, Provider,
};
use log::info;
use mockall_double::double;
use tungstenite::Message;

#[double]
use crate::socket::Sock;

pub struct Bitstamp {
    config: ConfigRef,
    socket: Sock,
}

impl Drop for Bitstamp {
    fn drop(&mut self) {
        info!("bitstamp disconnect");
        self.socket.close();
    }
}

impl Provider for Bitstamp {
    fn name(&self) -> &'static str {
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
                let orderbook = response.orderbook();

                let mut summary = Summary::default();

                for order in orderbook.asks() {
                    let level = Level {
                        exchange: String::from(self.name()),
                        price: order[0].parse()?,
                        amount: order[1].parse()?,
                    };
                    summary.asks.push(level);
                }

                for order in orderbook.bids() {
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

        let socket = Sock::new(url).expect("failed to connect to bitstamp");

        Self { config, socket }
    }

    fn write(&self, request: Message) -> Result<()> {
        self.socket.write_message(request)
    }

    fn read(&self) -> Result<Message> {
        self.socket.read_message()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::socket::MockSock;
    use anyhow::bail;
    use common::config::Config;
    use mockall::predicate::eq;
    use url::Url;

    #[test]
    fn test_connect_well() {
        let context = MockSock::new_context();

        context
            .expect()
            .with(eq(Url::parse("wss://ws.bitstamp.net").unwrap()))
            .returning(|_| {
                let mut mocked = MockSock::default();
                mocked.expect_close().once();
                Ok(mocked)
            });

        let _provider = Bitstamp::new(Config::as_ref());
    }

    #[test]
    #[should_panic]
    fn test_connect_fail() {
        let context = MockSock::new_context();

        context
            .expect()
            .returning(|_url| bail!("Failed to connect"));

        let _provider = Bitstamp::new(Config::as_ref());
    }

    #[test]
    fn test_name() {
        let context = MockSock::new_context();
        context.expect().returning(|_url| {
            let mut mocked = MockSock::default();
            mocked.expect_close().once();
            Ok(mocked)
        });

        let provider = Bitstamp::new(Config::as_ref());
        assert_eq!(provider.name(), "Bitstamp");
    }

    #[test]
    fn test_subscribe_well() {
        let context = MockSock::new_context();

        context.expect().returning(|_url| {
            let mut mocked = MockSock::default();

            mocked
                .expect_write_message()
                .withf(|message| {
                    message.to_string().eq("{\"event\":\"bts:subscribe\",\"data\":{\"channel\":\"order_book_ethbtc\"}}")
                })
                .returning(|_| Ok(()));

            mocked
                .expect_read_message()
                .returning(|| Ok(Message::Text(String::from("{{}}"))));

            mocked.expect_close().once();
            Ok(mocked)
        });

        let provider = Bitstamp::new(Config::as_ref());

        assert!(provider.subscribe().is_ok());
    }

    #[test]
    fn test_subscribe_write_fail() {
        let context = MockSock::new_context();
        context.expect().returning(|_url| {
            let mut mocked = MockSock::default();

            mocked
                .expect_write_message()
                .returning(|_| bail!("Failed to write"));

            mocked.expect_read_message().never();

            mocked.expect_close().once();
            Ok(mocked)
        });

        let provider = Bitstamp::new(Config::as_ref());

        assert!(provider.subscribe().is_err());
    }

    #[test]
    fn test_subscribe_read_fail() {
        let context = MockSock::new_context();
        context.expect().returning(|_url| {
            let mut mocked = MockSock::default();

            mocked.expect_write_message().returning(|_| Ok(()));

            mocked
                .expect_read_message()
                .returning(|| bail!("Failed to write"));

            mocked.expect_close().once();
            Ok(mocked)
        });

        let provider = Bitstamp::new(Config::as_ref());

        assert!(provider.subscribe().is_err());
    }

    #[test]
    fn test_unsubscribe_well() {
        let context = MockSock::new_context();

        context.expect().returning(|_url| {
            let mut mocked = MockSock::default();

            mocked
                .expect_write_message()
                .withf(|message| {
                    message.to_string().eq("{\"event\":\"bts:unsubscribe\",\"data\":{\"channel\":\"order_book_ethbtc\"}}")
                })
                .returning(|_| Ok(()));

            mocked
                .expect_read_message()
                .returning(|| Ok(Message::Text(String::from("{{}}"))));

            mocked.expect_close().once();
            Ok(mocked)
        });

        let provider = Bitstamp::new(Config::as_ref());

        assert!(provider.unsubscribe().is_ok());
    }

    #[test]
    fn test_summary_well() -> Result<()> {
        let context = MockSock::new_context();

        context.expect().returning(|_url| {
            let mut mocked = MockSock::default();

            mocked.expect_read_message().returning(|| {
                Ok(Message::Text(String::from(
                    r#"{"data":{"timestamp":"1682624742","microtimestamp":"1682624742462361","bids":[["0.06466182","0.50000000"],["0.06465586","0.77986816"]],"asks":[["0.06468051","0.50000000"],["0.06468374","0.40000000"]]},"channel":"","event":""}"#,
                )))
            });

            mocked.expect_close().once();
            Ok(mocked)
        });

        let provider = Bitstamp::new(Config::as_ref());

        let summary = provider.summary()?;

        assert_eq!(summary.bids.len(), 2);

        assert_eq!(summary.bids[0].exchange, "Bitstamp");
        assert_eq!(summary.bids[0].price, 0.06466182);
        assert_eq!(summary.bids[0].amount, 0.50000000);

        assert_eq!(summary.bids[1].exchange, "Bitstamp");
        assert_eq!(summary.bids[1].price, 0.06465586);
        assert_eq!(summary.bids[1].amount, 0.77986816);

        assert_eq!(summary.asks.len(), 2);

        assert_eq!(summary.asks[0].exchange, "Bitstamp");
        assert_eq!(summary.asks[0].price, 0.06468051);
        assert_eq!(summary.asks[0].amount, 0.50000000);

        assert_eq!(summary.asks[1].exchange, "Bitstamp");
        assert_eq!(summary.asks[1].price, 0.06468374);
        assert_eq!(summary.asks[1].amount, 0.40000000);

        Ok(())
    }
}
