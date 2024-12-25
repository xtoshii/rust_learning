use std::collections::HashMap;
use tracing::span::{Attributes, Record};
use tracing::{info, Id, Subscriber};
use tracing::field::{Field, Visit};
use tracing_subscriber::Layer;
use tracing_subscriber::layer::Context;

pub struct CustomAttrLayer;

impl<S> Layer<S> for CustomAttrLayer
where
    S: Subscriber,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, _id: &Id, _ctx: Context<'_, S>) {
        let mut visitor = TraceCtxVisitor::new();
        attrs.record(&mut visitor);

        info!("on_new_span: {:?}", visitor.ctx);
    }

    fn on_record(&self, _span: &Id, _values: &Record<'_>, _ctx: Context<'_, S>) {
        let mut visitor = TraceCtxVisitor::new();
        _values.record(&mut visitor);

        info!("on_record: {:?}", visitor.ctx);
    }

    fn on_exit(&self, _id: &Id, _ctx: Context<'_, S>) {
        info!("on_exit");
    }

    fn on_close(&self, _id: Id, _ctx: Context<'_, S>) {
        info!("on_close");
    }
}




#[derive(Debug)]
pub struct TraceCtxVisitor {
    pub ctx: HashMap<String, String>,
}

impl TraceCtxVisitor {
    pub fn new() -> Self {
        TraceCtxVisitor { ctx: HashMap::new() }
    }
}


impl Visit for TraceCtxVisitor {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.ctx.insert(field.name().to_string(), format!("{:?}", value));
    }
}