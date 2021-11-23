//Based on https://github.com/open-telemetry/opentelemetry-rust/tree/main/examples/basic

use opentelemetry::global;
use opentelemetry::global::shutdown_tracer_provider;
use opentelemetry::sdk::trace::Config;
use opentelemetry::sdk::{trace as sdktrace, Resource};
use opentelemetry::trace::TraceError;
use opentelemetry::{
    trace::{TraceContextExt, Tracer}, Key, KeyValue,
};
use std::error::Error;

fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
    opentelemetry_jaeger::new_pipeline()
        .with_service_name("trace-demo")
        .with_trace_config(Config::default().with_resource(Resource::new(vec![
            KeyValue::new("exporter", "otlp-jaeger"),
        ])))
        .install_batch(opentelemetry::runtime::Tokio)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // By binding the result to an unused variable, the lifetime of the variable
    // matches the containing block, reporting traces and metrics during the whole
    // execution.
    let _tracer = init_tracer()?;

    let tracer = global::tracer("ex.com/basic");

    tracer.in_span("operation", |cx| {
        let span = cx.span();
        span.add_event(
            "Nice operation!".to_string(),
            vec![Key::new("bogons").i64(100)],
        );

        tracer.in_span("Sub operation...", |cx| {
            let span = cx.span();

            span.add_event("Sub span event".to_string(), vec![]);
        });
    });

    shutdown_tracer_provider(); // sending remaining spans.

    Ok(())
}
