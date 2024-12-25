use tracing::{field, info, instrument, span, Level, Span};
use tracing::subscriber::set_global_default;

pub mod tracing_config;
fn main() {
    let (subscriber, guards) = crate::tracing_config::setup_logging();
    set_global_default(subscriber).expect("setting default subscriber failed");

    // main函数中需要持有 guard, 否则日志可能会丢失
    let _guard = guards;
    info!("Hello, Tracing");

    // test_record1();

    test_record2();
}

pub fn test_record1(){
    let span = span!(Level::INFO, "test_record1");

    let _enter = span.enter();

    info!("before:{:?}",span);

    span.record("trace_id", "456");

    info!("after:{:?}",span);
}


pub fn test_record2(){
    let span = span!(Level::INFO, "test_record2", trace_id="123");

    let _enter = span.enter();

    info!("before_test_record_2:{:?}",span);

    test_record3();

    info!("after_test_record_2:{:?}",span)
}


pub fn test_record3(){
    let span  = Span::current();
    span.record("trace_id", "456");
}