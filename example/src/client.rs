//! # Example gRPC Client with fastrace Tracing
//!
//! This example demonstrates how to use fastrace with a Tonic gRPC client.
//! The client sends ping requests to a server and traces the requests using fastrace.

pub mod ping {
    tonic::include_proto!("grpc.examples.ping");
}

use fastrace::collector::Config;
use fastrace::collector::ConsoleReporter;
use fastrace::prelude::*;
use ping::PingRequest;
use ping::ping_client::PingClient;
use tonic::transport::Channel;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the fastrace reporter with a console reporter.
    fastrace::set_reporter(ConsoleReporter, Config::default());

    {
        // Create a root span for the client operation.
        let root = Span::root("client".to_string(), SpanContext::random());
        let _g = root.set_local_parent();

        ping().await?;
    }

    // Flush any remaining traces before the program exits.
    fastrace::flush();

    Ok(())
}

/// Sends a ping request to the server with tracing.
///
/// This function demonstrates how to set up a Tonic gRPC client with the fastrace
/// middleware for tracing and context propagation.
#[fastrace::trace]
async fn ping() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the gRPC server.
    let channel = Channel::from_static("[::1]:50051").connect().await?;

    // Apply the fastrace client layer to the channel.
    // This layer will add trace context to outgoing requests.
    let channel = ServiceBuilder::new()
        .layer(fastrace_tonic::FastraceClientLayer::default())
        .service(channel);

    // Create the client with the enhanced channel.
    let mut client = PingClient::new(channel);

    // Create and send a request.
    let request = tonic::Request::new(PingRequest {
        message: "ping".to_string(),
    });
    let response = client.ping(request).await?;

    println!("{}", response.get_ref().message);

    Ok(())
}
