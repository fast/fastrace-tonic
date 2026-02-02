# CHANGELOG

All significant changes to this project will be documented in this file.

## Unreleased

## v0.2.0

### Breaking Changes

* `FastraceServerLayer` now requires construction; use `FastraceServerLayer::default()` or `with_span_context_extractor`.

### New Features

* Added a configurable span context extractor; default reads `traceparent` and starts a new trace when missing or invalid.
