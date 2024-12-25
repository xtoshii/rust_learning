use std::path::PathBuf;
use chrono::Local;
use tracing::{info, span, Level, Subscriber};
use tracing::subscriber::set_global_default;
use tracing_appender::non_blocking::NonBlocking;
use tracing_appender::rolling::daily;
use tracing_subscriber::{fmt, Layer, Registry};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::layer::SubscriberExt;
use crate::custom_layer::CustomAttrLayer;

pub mod tracing_config;
pub mod custom_layer;
fn main() {

    let (subscriber, guards) = setup_logging();
    set_global_default(subscriber).expect("setting default subscriber failed");

    // main函数中需要持有 guard, 否则日志可能会丢失
    let _guard = guards;
    info!("Hello, Tracing");


    test_span();

}

fn test_span(){
    let span = span!(Level::INFO, "test_span", trace_id="123");

    let _enter = span.enter();

    span.record("trace_id", "456");
}


pub fn setup_logging() -> (
    impl Subscriber + Send + Sync,
    Vec<tracing_appender::non_blocking::WorkerGuard>,
) {
    // 创建 app.log 的按天滚动日志写入器
    let app_log_dir = PathBuf::from("./log");
    let app_rolling_appender = daily(app_log_dir.clone(), "tracing_test.log");
    let (non_blocking_appender, guard_appender) = NonBlocking::new(app_rolling_appender);

    // 设置日志输出时的格式，例如，是否包含日志级别、是否包含日志来源位置、设置日志的时间格式
    // 参考: https://docs.rs/tracing-subscriber/0.3.3/tracing_subscriber/fmt/struct.SubscriberBuilder.html#method.with_timer
    let format =
        tracing_subscriber::fmt::format().with_level(true).with_target(true).with_timer(tracing_config::LocalTimer);

    // 创建 fmt 层
    let app_layer = fmt::layer()
        .with_writer(non_blocking_appender)
        .with_ansi(false)
        .event_format(format)
        .with_target(false)
        .with_filter(tracing_subscriber::filter::filter_fn(|metadata| {
            metadata.target() != "detail"
        }));

    // 将两个层组合在一起
    let subscriber = Registry::default()
        .with(app_layer)
        .with(CustomAttrLayer);


    (subscriber, vec![guard_appender])
}