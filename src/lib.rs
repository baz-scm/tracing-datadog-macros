//! Collection of tracing macros
//!
//! # Example
//!
//! ```rust
//! use tracing_datadog_macros::instrument_web;
//!
//! #[instrument_web(skip(self))]
//! async fn process_data(&self, data: Data) {
//!     // ...
//! }
//! ```

#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

mod attr;

use proc_macro::TokenStream;
use quote::quote;

const HTTP: &str = r#"span.type = "http""#;
const QUEUE_CONSUMER: &str = r#"span.type = "queue", span.kind = "consumer", messaging.system = "pgmq""#;
const QUEUE_PRODUCER: &str = r#"span.type = "queue", span.kind = "producer", messaging.system = "pgmq""#;
const SQL: &str = r#"span.type = "sql", service.name = "database", db.system = "postgresql""#;
const WEB: &str = r#"span.type = "web""#;

#[proc_macro_attribute]
pub fn instrument_custom(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_instrumented_method(attr, item, "", Some(vec!["service_name"])).into()
}

#[proc_macro_attribute]
pub fn instrument_http(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_instrumented_method(attr, item, HTTP, Some(vec!["service_name"])).into()
}

#[proc_macro_attribute]
pub fn instrument_queue_consumer(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_instrumented_method(attr, item, QUEUE_CONSUMER, Some(vec!["service_name"])).into()
}

#[proc_macro_attribute]
pub fn instrument_queue_producer(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_instrumented_method(attr, item, QUEUE_PRODUCER, Some(vec!["service_name"])).into()
}

#[proc_macro_attribute]
pub fn instrument_sql(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_instrumented_method(attr, item, SQL, None).into()
}

#[proc_macro_attribute]
pub fn instrument_web(attr: TokenStream, item: TokenStream) -> TokenStream {
    generate_instrumented_method(attr, item, WEB, None).into()
}

#[allow(clippy::needless_pass_by_value)]
fn generate_instrumented_method(
    attr: TokenStream,
    item: TokenStream,
    extra_fields: &str,
    required_attrs: Option<Vec<&str>>,
) -> proc_macro2::TokenStream {
    let attr: proc_macro2::TokenStream = attr.into();
    let item: proc_macro2::TokenStream = item.into();
    let extended_attr: proc_macro2::TokenStream =
        attr::extend_fields(attr.to_string().as_str(), extra_fields, required_attrs)
            .parse()
            .unwrap();

    quote!(
        #[tracing::instrument(#extended_attr)]
        #item
    )
}
