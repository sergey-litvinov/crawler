use log::{info, error};
use anyhow::{Result};
use url::Url;

use crate::settings::TelegramSettings;
use crawler::{Notifier, NotifyData};

pub struct TelegramNotifier{
    chat_id: String,
    token: String
}

#[derive(Deserialize)]
struct TelegramResponse
{
    ok: bool,
    description: Option<String>
}

impl TelegramNotifier {
    pub fn new(config: TelegramSettings) -> Result<Self> {
        Ok(TelegramNotifier {
            chat_id: config.chat_id,
            token: config.token
        })
    }
}

impl Notifier for TelegramNotifier{
    fn notify(&self, data: &NotifyData) -> Result<()> {

        // we also could use teloxide as Telegram API SDK but it has old tokio references and bring dependency issues
        let message = format!("{}: {}", data.site_name, data.message);
        let query = format!("https://api.telegram.org/bot{}/sendMessage", self.token);
        let url =
            Url::parse_with_params(&query,
                                   &[("chat_id", &self.chat_id),
                                       ("text", &message)])
                .expect("Failed to parse telegram text");

        info!("Sending request to Telegram");
        let response = reqwest::blocking::get(url)?.json::<TelegramResponse>()?;

        if !response.ok {
            error!("Telegram API response wasn't successful {}", response.description.unwrap());
        }

        Ok(())
    }
}
