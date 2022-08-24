use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Debug;
use serde_json::{json, Value};
use tracing::{Event, Id};
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Record};
use tracing_subscriber::Layer;
use tracing_subscriber::layer::Context;

pub struct CustomLayer;

impl<S> Layer<S> for CustomLayer
    where S: tracing::Subscriber,
          S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup> {
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        // let span = ctx.span(id).unwrap();
        // println!("Got on_new_span!");
        // println!("  level={}", span.metadata().level());
        // println!("  target={}", span.metadata().target());
        // println!("  name={}", span.metadata().name());
        //
        // let mut visitor = PrintlnVisitor;
        // attrs.record(&mut visitor);

        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut fields);
        attrs.record(&mut visitor);

        let storage = CustomFieldStorage(fields);

        let span = ctx.span(id).unwrap();
        let mut extensions = span.extensions_mut();
        extensions.insert::<CustomFieldStorage>(storage);
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        // What's the parent span look like?
        let parent_span = ctx.event_span(event).unwrap();
        println!("parent span");
        print!("    name={}", parent_span.name());
        print!("    target={}", parent_span.metadata().target());
        println!();

        // All of the span context
        let scope = ctx.event_scope(event).unwrap();
        println!("Got event!");
        let mut spans = vec![];
        for span in scope.from_root() {
            // println!("an ancestor span");
            // println!("  name={}", span.name());
            // println!("  target={}", span.metadata().target());
            let extensions = span.extensions();
            let storage = extensions.get::<CustomFieldStorage>().unwrap();
            let field_data: &BTreeMap<String, Value> = &storage.0;
            // println!("  span");
            // println!("      target={:?}", span.metadata().target());
            // println!("      target={:?}", span.metadata().name());
            // println!("      stored fields={:?}", storage);
            spans.push(serde_json::json!({
                "target": span.metadata().target(),
                "name": span.name(),
                "level": format!("{:?}", span.metadata().level()),
                "fields": field_data,
            }));
        }

        // The fields of the event
        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut fields);
        event.record(&mut visitor);

        // And create our output
        let output = json!({
                "target": event.metadata().target(),
                "name": event.metadata().name(),
                "level": format!("{:?}", event.metadata().level()),
                "fields": fields,
            "spans":spans,
            });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }

    fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        let span = ctx.span(id).unwrap();
        let mut extensions_mut = span.extensions_mut();
        let custom_field_storage: &mut CustomFieldStorage = extensions_mut.get_mut().unwrap();
        let json_data: &mut BTreeMap<String, serde_json::Value> = &mut custom_field_storage.0;

        let mut visitor = JsonVisitor(json_data);
        values.record(&mut visitor);
    }
}

#[derive(Debug)]
struct CustomFieldStorage(BTreeMap<String, serde_json::Value>);

struct JsonVisitor<'a>(&'a mut BTreeMap<String, Value>);

impl<'a> Visit for JsonVisitor<'a> {
    fn record_f64(&mut self, field: &Field, value: f64) {
        self.0.insert(field.name().to_string(), json!(value));
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.0.insert(field.name().to_string(), json!(value));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.0.insert(field.name().to_string(), json!(value));
    }

    fn record_i128(&mut self, field: &Field, value: i128) {
        self.0.insert(field.name().to_string(), json!(value));
    }

    fn record_u128(&mut self, field: &Field, value: u128) {
        self.0.insert(field.name().to_string(), json!(value));
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.0.insert(field.name().to_string(), json!(value));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.0.insert(field.name().to_string(), json!(value));
    }

    fn record_error(&mut self, field: &Field, value: &(dyn Error + 'static)) {
        self.0.insert(field.name().to_string(), json!(value.to_string()));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        self.0.insert(field.name().to_string(), json!(format!("{:?}", value)));
    }
}

struct PrintlnVisitor;

impl Visit for PrintlnVisitor {
    fn record_f64(&mut self, field: &Field, value: f64) {
        println!("  field={} value={}", field.name(), value);
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        println!("  field={} value={}", field.name(), value);
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        println!("  field={} value={}", field.name(), value);
    }

    fn record_i128(&mut self, field: &Field, value: i128) {
        println!("  field={} value={}", field.name(), value);
    }

    fn record_u128(&mut self, field: &Field, value: u128) {
        println!("  field={} value={}", field.name(), value);
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        println!("  field={} value={}", field.name(), value);
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        println!("  field={} value={}", field.name(), value);
    }

    fn record_error(&mut self, field: &Field, value: &(dyn Error + 'static)) {
        println!("  field={} value={}", field.name(), value);
    }

    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        println!("  field={} value={:?}", field.name(), value);
    }
}
