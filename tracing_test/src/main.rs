use crate::tracing_config::setup_logging;
use tracing::subscriber::set_global_default;
use tracing::{info, instrument, span, Instrument, Level, Span};

pub mod tracing_config;
pub mod span_record;
pub mod custom_layer;

#[tokio::main]
async  fn main() {
    let (subscriber, guards) = setup_logging();
    set_global_default(subscriber).expect("setting default subscriber failed");

    // main函数中需要持有 guard, 否则日志可能会丢失
    let _guard = guards;
    info!("Hello, Tracing");

    // let span = span!(Level::INFO, "test_span1");
    //
    // // 进入span，
    // let _enter = span.enter();
    // sync_test_span();

    // async_test_span().await

    async_test_span3().await

}

#[instrument]
pub async  fn async_test_span3(){
    let r = Span::current();
    println!("async_test_span3: {:?}", r);
}

pub fn sync_test_span() {
    let r = Span::current();
    println!("sync_test_span: {:?}", r);
}

async fn async_test_span() {
    let span = span!(Level::INFO, "async_test_span");
    async_test_span2().instrument(span).await;
}

async fn async_test_span2() {
    let r = Span::current();
    println!("async_test_span2: {:?}", r);
}



