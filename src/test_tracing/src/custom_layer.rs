use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Debug;
use serde_json::{json, Value};
use tracing::Event;
use tracing::field::{Field, Visit};
use tracing_subscriber::Layer;
use tracing_subscriber::layer::Context;

pub struct CustomLayer;

impl<S> Layer<S> for CustomLayer where S: tracing::Subscriber {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        println!("Got event!");
        println!("  level={:?}", event.metadata().level());
        println!("  target={:?}", event.metadata().target());
        println!("  name={:?}", event.metadata().name());
        // for field in event.fields() {
        //     println!("  field={}", field.name());
        // }

        // let mut visitor = PrintlnVisitor;
        // event.record(&mut visitor);

        // Convert the values into a JSON object
        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut fields);
        event.record(&mut visitor);

        // Output the event in JSON
        let output = json!({
           "target": event.metadata().target(),
           "name": event.metadata().name(),
           "level": format!("{:?}", event.metadata().level()),
            "fields": fields,
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
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