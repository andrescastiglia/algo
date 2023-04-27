use clap::Parser;
use std::{net::SocketAddr, sync::Arc};

pub type ConfigRef = Arc<Config>;

/// it's just an assessment
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Pair
    #[arg(long, default_value = "ethbtc")]
    pair: String,

    #[cfg(feature = "binance")]
    /// Binance URL
    #[arg(
        long,
        value_parser(url::Url::parse),
        default_value = "wss://stream.binance.com:9443/ws/"
    )]
    binance_url: url::Url,

    #[cfg(feature = "bitstamp")]
    /// Bitstamp URL
    #[arg(
        long,
        value_parser(url::Url::parse),
        default_value = "wss://ws.bitstamp.net"
    )]
    bitstamp_url: url::Url,

    #[arg(long, default_value_t = 10)]
    /// Top rows
    top: usize,

    #[arg(long, default_value = "[::1]:50051")]
    // Local bind
    local_bind: SocketAddr,

    #[arg(long, default_value_t = false)]
    // CLI
    cli: bool,
}

impl Config {
    pub fn as_ref() -> ConfigRef {
        let def: Vec<std::ffi::OsString> = vec![];
        Arc::new(Self::parse_from(def))
    }

    pub fn pair(&self) -> &str {
        self.pair.as_str()
    }

    #[cfg(feature = "binance")]
    pub const fn binance_url(&self) -> &url::Url {
        &self.binance_url
    }

    #[cfg(feature = "bitstamp")]
    pub const fn bitstamp_url(&self) -> &url::Url {
        &self.bitstamp_url
    }

    pub fn top(&self) -> usize {
        self.top
    }

    pub fn local_bind(&self) -> SocketAddr {
        self.local_bind
    }

    pub fn cli(&self) -> bool {
        self.cli
    }
}
