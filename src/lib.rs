#![doc = include_str!("../README.md")]

use std::task::Context;
use std::task::Poll;

use fastrace::prelude::*;
use http::HeaderValue;
use http::Request;
use tower_layer::Layer;
use tower_service::Service;

/// The standard [W3C Trace Context](https://www.w3.org/TR/trace-context/) header name for passing trace information.
///
/// This is the header key used to propagate trace context between services according to
/// the W3C Trace Context specification.
pub const TRACEPARENT_HEADER: &str = "traceparent";

/// Server layer for intercepting and processing trace context in incoming requests.
///
/// This layer extracts tracing context from incoming requests and creates a new span
/// for each request. Add this to your tonic server to automatically handle trace context
/// propagation.
#[derive(Clone)]
pub struct FastraceServerLayer;

impl<S> Layer<S> for FastraceServerLayer {
    type Service = FastraceServerService<S>;

    fn layer(&self, service: S) -> Self::Service {
        FastraceServerService { service }
    }
}

/// Server-side service that handles trace context propagation.
///
/// This service extracts trace context from incoming requests and creates
/// spans to track the request processing. It wraps the inner service and augments
/// it with tracing capabilities.
#[derive(Clone)]
pub struct FastraceServerService<S> {
    service: S,
}

impl<S, Body> Service<Request<Body>> for FastraceServerService<S>
where S: Service<Request<Body>>
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = fastrace::future::InSpan<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let headers = req.headers();
        let parent = headers.get(TRACEPARENT_HEADER).and_then(|traceparent| {
            SpanContext::decode_w3c_traceparent(traceparent.to_str().ok()?)
        });

        let span = if let Some(parent) = parent {
            Span::root(req.uri().to_string(), parent)
        } else {
            Span::noop()
        };

        self.service.call(req).in_span(span)
    }
}

/// Client layer for injecting trace context into outgoing requests.
///
/// This layer adds the current trace context to outgoing requests,
/// allowing the receiving service to continue the same trace. Add this
/// to your tonic client to automatically propagate trace context.
#[derive(Clone)]
pub struct FastraceClientLayer;

impl<S> Layer<S> for FastraceClientLayer {
    type Service = FastraceClientService<S>;

    fn layer(&self, service: S) -> Self::Service {
        FastraceClientService { service }
    }
}

/// Client-side service that handles trace context propagation.
///
/// This service injects the current trace context into outgoing requests,
/// allowing distributed tracing across service boundaries.
#[derive(Clone)]
pub struct FastraceClientService<S> {
    service: S,
}

impl<S, Body> Service<Request<Body>> for FastraceClientService<S>
where S: Service<Request<Body>>
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        if let Some(current) = SpanContext::current_local_parent() {
            req.headers_mut().insert(
                TRACEPARENT_HEADER,
                HeaderValue::from_str(&current.encode_w3c_traceparent()).unwrap(),
            );
        }
        
        self.service.call(req)
    }
}
