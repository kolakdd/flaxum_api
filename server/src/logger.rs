use tracing_subscriber::{
    filter::{self},
    fmt::{layer, time::ChronoLocal},
    prelude::*,
    registry,
};

use crate::config::Config;

pub fn init(config: &Config) -> Result<(), crate::Error> {
    dbg!("config: {}", config);
    let log_level = filter::LevelFilter::DEBUG;
    let env_filter = filter::EnvFilter::new("")
        .add_directive(log_level.into())
        .add_directive("sqlx::query=error".parse().unwrap())
        .add_directive("hyper=warn".parse().unwrap())
        .add_directive("reqwest=warn".parse().unwrap());

    let fmt_layer = layer()
        .with_target(true)
        .with_timer(ChronoLocal::rfc_3339())
        .with_filter(env_filter);

    registry().with(fmt_layer).init();

    Ok(())
}
