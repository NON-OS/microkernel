//! Host cover for the framing net.core reads requests through.
//!
//! The modules are the shipping ones, pulled in by path, so a test cannot
//! pass against a copy that has drifted from what the capsule runs.

#[path = "../../../src/device/batch_frames.rs"]
pub mod batch_frames;
pub mod protocol;
pub mod server;

#[cfg(test)]
mod batch_tests;
#[cfg(test)]
mod capacity_tests;
#[cfg(test)]
mod vectors;
