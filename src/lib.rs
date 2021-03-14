use anyhow::{Result};

#[derive(Debug)]
pub struct NotifyData {
    pub site_name: String,
    pub message: String,
}

pub trait Notifier {
    fn notify(&self, data: &NotifyData) -> Result<()>;
}

pub trait Parser {
    fn parse(&self) -> Result<Option<NotifyData>>;
}