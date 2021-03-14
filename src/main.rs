#[macro_use]
extern crate serde_derive;
extern crate config;
extern crate serde;

mod settings;
mod notifiers;
mod parsers;
mod scheduler;

use std::{
    thread,
    time::Duration,
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc
};
use crate::{
    settings::Settings,
    scheduler::Scheduler
};
use log::{info};
use error_chain::{error_chain};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() {

    init_logging();
    let running = Arc::new(AtomicBool::new(true));
    let tracker = running.clone();
    ctrlc::set_handler(move || {
        tracker.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    info!("Crawler is starting");
    crawler_main();

    info!("Crawler is started. Use Ctrl-C to stop it");
    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100))
    }
    // it's not that graceful shutdown since threads still might be running
    info!("Exiting...");
}

fn init_logging(){
    let env =
        env_logger::Env::default()
            .filter_or("RUST_LOG", "warn,crawler=info");
    env_logger::init_from_env(env);
}

fn crawler_main(){
    let settings = Settings::new().unwrap();

    info!("Loaded settings");

    for site in &settings.sites {
        info!("{:?} {:?}", site.endpoint, site.parser_type)
    }

    let scheduler = Scheduler::new(settings.sites, settings.telegram).unwrap();
    scheduler.start();
}
