use log::{trace, warn};
use anyhow::{Result};

use crawler::{Parser, Notifier, NotifyData};
use crate::{
    settings::*,
    parsers::html_parser::HtmlParser,
    parsers::json_parser::JsonParser,
    notifiers::telegram_notifier::TelegramNotifier
};
use std::{
    sync::Arc,
    thread,
    time::Duration
};

pub struct Scheduler{
    work_items:Vec<WorkItem>,
    pub notifier: Arc<TelegramNotifier>
}

struct WorkItem{
    site: Arc<Site>,
    notifier: Arc<TelegramNotifier>
}

impl Scheduler{
    pub fn new(sites: Vec<Site>, telegram: TelegramSettings) -> Result<Self> {
        let mut items: Vec<WorkItem> = vec![];
        let notifier = Arc::new(
            TelegramNotifier::new(telegram).unwrap());

        for site in sites {
            items.push(WorkItem{
                site: Arc::new(site),
                notifier: notifier.clone()
            })
        }


        Ok(Scheduler{
            work_items: items,
            notifier
        })
    }

    fn create(site: &Site) -> Box<dyn Parser>{
        return match site.parser_type {
            ParserType::Html => {
                Box::new(HtmlParser::new(site).expect("HtmlParser can't be created"))
            },
            ParserType::Json => {
                Box::new(JsonParser::new(site).expect("JsonParser can't be created"))
            }
        };
    }

    pub fn start(&self) {

        self.notifier.notify(&NotifyData{
           site_name:"Local".to_owned(), message: "The bot is starting".to_owned()
        }).expect("Notification should be sent");

        for item in &self.work_items {
            item.start();
            // sleep for some time to avoid so many requests the same time
            thread::sleep(Duration::from_secs(2));
        }
    }
}


impl WorkItem{
    fn start(&self){
        let local = self.site.clone();
        let notifier = self.notifier.clone();
        std::thread::spawn(move || {
            loop {

                trace!("Sending request to {:?}", local.endpoint);
                let parser = Scheduler::create(&local);
                let result = parser.parse();
                if result.is_ok()
                {
                    let opt = result.unwrap();
                    if opt.is_some()
                    {
                        notifier.notify(&opt.unwrap()).unwrap();
                    }
                }
                else
                {
                    warn!("Parsing is failed with {:?}", result.err().unwrap())
                }

                thread::sleep(Duration::from_secs(local.interval_seconds))
            }
        });
    }
}