use tracing_subscriber::{
    filter::{self},
    fmt::{layer, time::ChronoLocal},
    prelude::*,
    registry,
};

pub fn init() -> Result<(), crate::Error> {
    let log_level = filter::LevelFilter::DEBUG;
    let env_filter = filter::EnvFilter::new("")
        .add_directive(log_level.into())
        .add_directive("sqlx::query=error".parse().unwrap())
        .add_directive("hyper=warn".parse().unwrap())
        .add_directive("reqwest=warn".parse().unwrap())
        .add_directive("aws_sdk_s3=warn".parse().unwrap())
        .add_directive("aws_smithy_runtime=warn".parse().unwrap())
        .add_directive("aws_runtime=warn".parse().unwrap());

    let fmt_layer = layer()
        .with_target(true)
        .with_timer(ChronoLocal::rfc_3339())
        .with_filter(env_filter);

    registry().with(fmt_layer).init();

    Ok(())
}
