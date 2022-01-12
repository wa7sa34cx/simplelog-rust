#[macro_use] extern crate log;
extern crate simplelog;

use simplelog::*;

use std::fs::File;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("app.log").unwrap()),
        ]
    ).unwrap();

    trace!("trace");
    debug!("debug");
    info!("info");
    warn!("warn");
    error!("error");
}
