use std::env;
use ctor::ctor;
use log::LevelFilter;
use simple_logger::SimpleLogger;
const LNM_LOG_LEVEL: &str = "LNM_LOG_LEVEL";

/// Constructor called at library dynamic-load.
///
/// Not guaranteed to *always* run in a single-threaded context.  Application constructors
/// *could* create threads or lock mutexes before LD_PRELOAD loads our library.
///
#[ctor]
fn bootstrap() {
    let log_level = env::var(LNM_LOG_LEVEL).unwrap_or(String::from("info"));
    match log_level.to_lowercase().as_str() {
        "debug" => SimpleLogger::new().with_level(LevelFilter::Debug).init().unwrap(),
        _ => SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap(),
    }

    log::debug!("> bootstrap");
}


