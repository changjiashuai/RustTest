mod custom_layer;

use tracing::info;
use tracing_subscriber::prelude::*;
use crate::custom_layer::CustomLayer;

fn main() {
    tracing_subscriber::registry().with(CustomLayer).init();
    info!(a_bool = true, answer = 42, message = "first example");
}
