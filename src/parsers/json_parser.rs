use log::{trace, warn};
use anyhow::{Result, anyhow};
use reqwest::blocking::{Client};
use std::time::Duration;

use crate::settings::Site;
use crawler::{NotifyData, Parser};
use crate::parsers::hotline_parser::{HotlineParser};

pub struct JsonParser{
    site: Site,
    client: Client
}


impl JsonParser {
    pub fn new(site: &Site) -> Result<Self> {

        let client = reqwest::blocking::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.82 Safari/537.36")
            .connect_timeout(Duration::from_secs(30))
            .gzip(true)
            .build()
            .expect("Failed to create a client");

        Ok(JsonParser {
            site: site.clone(),
            client: client
        })
    }


    fn get_csrftoken(&self) -> Result<Option<String>>{

        if self.site.preflight_request.is_none() {
            return Ok(None);
        }

        let url = self.site.preflight_request.as_ref().unwrap().as_str();
        trace!("Sending request to {:?}", url);

        let response =
            self
                .client
                .get(url)
                .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
                .header("Accept-Language", "en-CA,en;q=0.9")
                .send()?
                .text()?;

        trace!("Got response of {:?} bytes", response.len());
        trace!("Parsing document for csrf token");
        let selector = self.site.csrf_token_selector.as_ref().unwrap();
        let document = scraper::Html::parse_document(&response);
        let query = scraper::Selector::parse(selector).unwrap();

        for element in document.select(&query) {

            let option = element.value().attr("content");
            if option.is_none() {
                return Err(anyhow!("Selector {:?} returned empty result", selector));
            }

            let value = option
                .unwrap()
                .to_string()
                .trim_end_matches("\"")
                .trim_start_matches("\"")
                .to_string();

            trace!("Found csrf token {:?}", value);

            if value.len() > 0 {
                return Ok(Some(value))
            }
        }

        Ok(None)
    }
}


impl Parser for JsonParser{
    fn parse(&self) -> Result<Option<NotifyData>> {

        let csrf_token = self.get_csrftoken();
        if csrf_token.is_err()
        {
            let error = csrf_token.err().unwrap();
            warn!("Failed to get csrf token {:?}", error);
            return Err(error);
        }

        let url = self.site.endpoint.as_str();
        trace!("Sending request to {:?}", url);

        let mut request =
            self.client
            .get(url)
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
            .header("Accept-Language", "en-CA,en;q=0.9");

        let token = csrf_token.unwrap();
        if token.is_some()
        {
            request = request.header(self.site.csrf_token_header.as_ref().unwrap(), token.unwrap());
        }

        let response =
            request
                .send()?
                .text()?;

        trace!("Got response of {:?} bytes", response.len());

        // shop specific parser since we need to handle json
        if &self.site.name != "Hotline" {
            return Ok(None)
        }

        let parser = HotlineParser{};

        return parser.parse_json(response, &self.site);
    }

}
