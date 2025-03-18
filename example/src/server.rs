//! # Example gRPC Server with fastrace Tracing
//!
//! This example demonstrates how to use fastrace with a Tonic gRPC server.
//! The server responds to ping requests and traces the requests using fastrace.

pub mod ping {
    tonic::include_proto!("grpc.examples.ping");
}

use fastrace::collector::Config;
use fastrace::collector::ConsoleReporter;
use ping::PingRequest;
use ping::PingResponse;
use ping::ping_server::Ping;
use ping::ping_server::PingServer;
use tonic::Request;
use tonic::Response;
use tonic::Status;
use tonic::transport::Server;

/// Simple ping service implementation.
#[derive(Debug, Default)]
pub struct MyPing {}

#[tonic::async_trait]
impl Ping for MyPing {
    /// Handles ping requests and responds with "pong".
    #[fastrace::trace]
    async fn ping(&self, _req: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        let reply = PingResponse {
            message: "pong".to_string(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the fastrace reporter with a console reporter.
    fastrace::set_reporter(ConsoleReporter, Config::default());

    // Build and start the server with the fastrace server layer.
    // This layer will extract trace context from incoming requests.
    Server::builder()
        .layer(fastrace_tonic::FastraceServerLayer)
        .add_service(PingServer::new(MyPing::default()))
        .serve("[::1]:50051".parse().unwrap())
        .await?;

    // Flush any remaining traces before the program exits.
    fastrace::flush();

    Ok(())
}
