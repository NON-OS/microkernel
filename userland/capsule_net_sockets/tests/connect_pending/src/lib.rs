//! Host cover for what the serve loop decides about a waiting connect.
//!
//! The module is the shipping one, pulled in by path, so a test cannot pass
//! against a copy that has drifted from what the capsule runs.

#[path = "../../../src/server/handlers/connect/pending/verdict.rs"]
pub mod verdict;

#[cfg(test)]
mod vectors;
