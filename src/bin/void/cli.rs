use clap::{App, Arg};

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const ABOUT: &str = env!("CARGO_PKG_DESCRIPTION");

pub fn create<'a>() -> App<'a, 'a> {
    App::new(APP_NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(ABOUT)
        .arg(
            Arg::with_name("version")
                .short("v")
                .long("version")
                .help("Print version info and exit")
                .conflicts_with("PATH"),
        )
        .arg(Arg::with_name("PATH").takes_value(true).required(false))
}
