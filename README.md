# fastrace-tonic

[![Crates.io](https://img.shields.io/crates/v/fastrace-tonic.svg?style=flat-square&logo=rust)](https://crates.io/crates/fastrace-tonic)
[![Documentation](https://img.shields.io/docsrs/fastrace-tonic?style=flat-square&logo=rust)](https://docs.rs/fastrace-tonic/)
[![MSRV 1.75.0](https://img.shields.io/badge/MSRV-1.75.0-green?style=flat-square&logo=rust)](https://www.whatrustisit.com)
[![CI Status](https://img.shields.io/github/actions/workflow/status/fast/fastrace-tonic/ci.yml?style=flat-square&logo=github)](https://github.com/fast/fastrace-tonic/actions)
[![License](https://img.shields.io/crates/l/fastrace-tonic?style=flat-square)](https://github.com/fast/fastrace-tonic/blob/main/LICENSE)

`fastrace-tonic` is a middleware library that connects [fastrace](https://crates.io/crates/fastrace), a distributed tracing library, with [tonic](https://crates.io/crates/tonic), a gRPC framework for Rust. This integration enables seamless trace context propagation across microservice boundaries in gRPC-based applications.

## What is Context Propagation?

Context propagation is a fundamental concept in distributed tracing that enables the correlation of operations spanning multiple services. When a request moves from one service to another, trace context information needs to be passed along, ensuring that all operations are recorded as part of the same trace.

`fastrace-tonic` implements the [W3C Trace Context](https://www.w3.org/TR/trace-context/) standard for propagating trace information between services. This ensures compatibility with other tracing systems that follow the same standard.

## Features

- 🔄 **Automatic Context Propagation**: Automatically inject trace context into outgoing gRPC requests.
- 🌉 **Seamless Integration**: Works seamlessly with the `fastrace` library for complete distributed tracing.
- 📊 **Full Compatibility**: Works with fastrace's collection and reporting capabilities.

## Installation

Add `fastrace-tonic` to your Cargo.toml:

```toml
[dependencies]
fastrace = "0.7"
fastrace-tonic = "0.1"
```

### Server Integration

Apply the `FastraceServerLayer` to your tonic server:

```rust
use fastrace_tonic::FastraceServerLayer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize fastrace reporter.
    fastrace::set_reporter(ConsoleReporter, Config::default());
    
    // Add FastraceServerLayer to your tonic server.
    Server::builder()
        .layer(FastraceServerLayer)
        .add_service(YourServiceServer::new(YourService::default()))
        .serve("[::1]:50051".parse()?)
        .await?;
    
    fastrace::flush();

    Ok(())
}
```

### Client Integration

Apply the `FastraceClientLayer` to your tonic client:

```rust
use fastrace_tonic::FastraceClientLayer;
use tower::ServiceBuilder;

async fn make_client_call() -> Result<(), Box<dyn std::error::Error>> {
    // Create channel with FastraceClientLayer.
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;
        
    let channel = ServiceBuilder::new()
        .layer(FastraceClientLayer)
        .service(channel);
        
    // Create client with the enhanced channel.
    let mut client = YourServiceClient::new(channel);
    
    // Make calls as usual.
    let response = client.your_method(Request::new(YourRequest {})).await?;
    
    Ok(())
}
```

## Example

Check out the [examples directory](https://github.com/fast/fastrace-tonic/tree/main/examples) for a complete ping/pong service example that demonstrates both client and server tracing.

To run the example:

1. Navigate to the examples directory:
    ```
    cd examples
    ```

2. Start the server:
   ```
   cargo run --bin server
   ```

3. In another terminal, run the client:
   ```
   cargo run --bin client
   ```

Both applications will output trace information showing the request flow, including the propagated context.

## How It Works

1. When a client makes a request, `FastraceClientLayer` detects if there's an active trace and adds a `traceparent` HTTP header with the trace context.
2. When a server receives the request, `FastraceServerLayer` extracts the trace context from the `traceparent` header and creates a new span as a child of the received context.
3. If no trace context is provided, the server creates a new root span.

This process ensures that all operations across services are properly connected in the resulting trace, providing visibility into the entire request lifecycle.

## License

This project is licensed under the [Apache-2.0](./LICENSE) license.
