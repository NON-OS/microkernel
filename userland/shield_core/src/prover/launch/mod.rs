//! The launch prover: a spend proved on this device against the live pool,
//! through the frozen `nox_prover`, and the words the pool is handed.

pub mod cache;
pub mod entropy;
pub mod fixture;
pub mod prove;
pub mod publics;
pub mod request;
pub mod seed;
pub mod tree;

#[cfg(test)]
#[path = "launch_test.rs"]
mod launch_test;
