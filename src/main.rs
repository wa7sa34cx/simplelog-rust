#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;

use std::fs::File;

mod submod;

fn main() {
    let config = ConfigBuilder::new()
        .set_time_format_str("%Y-%m-%d %H:%M:%S")
        .build();
    let config2 = config.clone();

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            config,
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Error,
            config2,
            File::create("app.log").unwrap(),
        ),
    ])
    .unwrap();

    trace!("trace");
    debug!("debug");
    info!("info");
    warn!("warn");
    error!("error");

    submod::run();
}
