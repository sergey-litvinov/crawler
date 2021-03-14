use log::{trace, warn};
use anyhow::{Result};
use reqwest::blocking::{Client};

use crate::settings::Site;
use crawler::{NotifyData, Parser};

pub struct HtmlParser{
    site: Site,
    client: Client
}


impl HtmlParser {
    pub fn new(site: &Site) -> Result<Self> {

        let client = reqwest::blocking::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.82 Safari/537.36")
            .gzip(true)
            .build()?;

        Ok(HtmlParser {
            site: site.clone(),
            client
        })
    }
}


impl Parser for HtmlParser{
    fn parse(&self) -> Result<Option<NotifyData>> {

        let url = self.site.endpoint.as_str();

        trace!("Sending request to {:?}", url);

        let response =
            self.client
                .get(url)
                .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
                .header("Accept-Language", "en-CA,en;q=0.9")
                .send()?
                .text()?;

        trace!("Got response of {:?} bytes", response.len());
        trace!("Parsing document");
        let selector = &self.site.selector.as_ref().unwrap();
        let document = scraper::Html::parse_document(&response);
        let query = scraper::Selector::parse(selector).unwrap();

        let vec = self.site.ignored_texts.as_ref().unwrap();
        let mut no_result = true;

        for element in document.select(&query) {

            no_result = false;
            let mut html = element.inner_html();

            for ignored  in vec.as_slice() {
                html = html.replace(ignored, "")
            }

            html = html.replace("\n", "").replace("\r", "").trim().to_string();

            if html.len() > 0 {
                trace!("Found html content {:?}", html);

                return Ok(
                    Some(
                        NotifyData{
                            message: format!("{:?} has different message: {:?}.", url, html),
                            site_name: self.site.name.to_string()
                    }))
            }
        }

        if no_result {
            warn!("{:?}. Selector {:?} returned empty result.", url, selector);
        }

        Ok(None)
    }
}
