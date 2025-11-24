#![warn(missing_docs)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::std_instead_of_alloc)]
#![forbid(unsafe_code)]

//! # blazon
//!
//! A minimal badge generate for Rust crate stats.
//!
//! This crate provides badge generation for dependency count and binary size, as a library or CLI.

// Re-export the core functionality
pub use blazon_core::{Metrics, blazon_debug, collect_metrics, generate_badges};
