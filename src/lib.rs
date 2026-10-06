//! The Rust SDK for keel.
//!
//! keel is a durable execution engine. One `keeld` process journals each
//! step of a workflow and calls your service over HTTP. This crate wraps
//! that protocol, so a Rust worker and a Rust client need no raw HTTP or
//! WebSocket code.
//!
//! Early work. The API still changes. Read the engine docs before you
//! rely on it.
//!
//! No public API yet. The client and the worker arrive in the first
//! release.
