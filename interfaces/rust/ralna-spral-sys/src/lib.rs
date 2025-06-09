//! # RALNA SPRAL Native Bindings

// Prevents warnings when non-idiomatic Rust style is used.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(not(doc))]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
