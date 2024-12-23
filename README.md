<div align="center">
   <img align="center" width="128px" src="https://avatars.githubusercontent.com/u/140384842?s=200&v=4" />
   <h1 align="center"><b>tracing-datadog-macros</b></h1>
   <p align="center">
      Simplify Datadog tracing in Rust with specialized macros for enhanced code observability.
      <br />
      <a href="https://github.com/baz-scm/"><strong>Baz on GitHub »</strong></a>
      <br />
      <br />
      <b>Install via Cargo</b>
      <br />
      <code>cargo add tracing-datadog-macros</code>
      <br />
      <br />
      <b>Libraries Available</b>
      <br />
      Rust: <a href="https://github.com/baz-scm/tracing-datadog-macros">tracing-datadog-macros</a>
   </p>
</div>

---

![Crates.io](https://img.shields.io/crates/v/tracing-datadog-macros) 
![Rust](https://img.shields.io/badge/rust-2021-blue)
![Release](https://github.com/baz-scm/tracing-datadog-macros/workflows/Release/badge.svg)

## 🚀 What is `tracing-datadog-macros`?

`tracing-datadog-macros` is a Rust library providing a collection of procedural macros to simplify and enhance Datadog tracing. It automates span creation and enriches them with context-critical fields, making tracing more actionable and efficient for distributed systems.

With `tracing-datadog-macros`, your observability improves significantly, offering you crystal-clear insights into your application's behavior.

### Available Macros

`instrument_custom`: General-purpose instrumentation with custom attributes.

`instrument_http`: Predefined for HTTP spans.

`instrument_queue_consumer`: Optimized for queue consumers.

`instrument_queue_producer`: For queue producers.

`instrument_sql`: SQL-related spans.

`instrument_web`: Specialized for web spans.

Parse and validate attributes provided by the user.
Add required fields like span.type, service.name, and others.
Integrate seamlessly with #[tracing::instrument].


---
## Install

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
tracing-datadog-macros = "0.0.2"
```

## Usage
Example
```rust
use tracing_datadog_macros::instrument_web;

#[instrument_web(skip(self))]
async fn process_data(&self, data: Data) {
    // Your logic here
}
```

# 🔗 Learn More
Falken-Trace-Go library: https://github.com/baz-scm/falken-trace-go

Falken-Trace-py library: https://github.com/baz-scm/falken-trace-py

Blog post: [Extending OpenTelemetry to Pinpoint Code Elements](https://baz.co/resources/extending-opentelemetry-to-pinpoint-code-elements-our-journey-to-close-the-gap)
