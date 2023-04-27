use anyhow::{Context, Result};
use parking_lot::RwLock;
use std::net::TcpStream;
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use url::Url;

#[cfg(test)]
use mockall::automock;

pub struct Sock {
    socket: RwLock<WebSocket<MaybeTlsStream<TcpStream>>>,
}

#[cfg_attr(test, automock)]
impl Sock {
    #[cfg_attr(not(test), inline)]
    pub fn new(url: &Url) -> Result<Self> {
        let (socket, _) = connect(url)?;
        Ok(Self {
            socket: RwLock::new(socket),
        })
    }

    #[cfg_attr(not(test), inline)]
    pub fn close(&self) {
        self.socket.write().close(None).ok();
    }

    #[cfg_attr(not(test), inline)]
    pub fn write_message(&self, message: Message) -> Result<()> {
        self.socket
            .write()
            .write_message(message)
            .with_context(|| "Failed to write message")
    }

    #[cfg_attr(not(test), inline)]
    pub fn read_message(&self) -> Result<Message> {
        self.socket
            .write()
            .read_message()
            .with_context(|| "Failed to read message")
    }
}
