use crate::orderbook::Summary;
use anyhow::Result;

pub trait Provider: Sync + Send {
    fn name(&self) -> &str;
    fn subscribe(&self) -> Result<()>;
    fn unsubscribe(&self) -> Result<()>;
    fn summary(&self) -> Result<Summary>;
}
