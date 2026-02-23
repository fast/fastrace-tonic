# CHANGELOG

All significant changes to this project will be documented in this file.

## Unreleased

## v0.2.0

### Breaking Changes

* `FastraceServerLayer` now requires construction `FastraceServerLayer::default()`.

### New Features

* Added a configurable span context extractor; default reads `traceparent` and starts a new trace when missing or invalid.
