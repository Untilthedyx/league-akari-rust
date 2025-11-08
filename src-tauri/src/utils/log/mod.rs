use tracing::subscriber::set_global_default;
use tracing_subscriber::{filter::LevelFilter, fmt, prelude::*, registry};

pub fn init_logger() {
    let subscriber = registry().with(LevelFilter::INFO).with(
        fmt::layer()
            .with_target(true)
            .with_timer(fmt::time::LocalTime::rfc_3339())
            .with_ansi(true),
    );
    set_global_default(subscriber).unwrap_or_else(|_| panic!("init logger error"));
}
