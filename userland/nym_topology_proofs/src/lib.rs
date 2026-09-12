// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for mixnet route selection. Includes the real capsule source
//! via `#[path]` and drives it over the candidate counts the live network
//! actually publishes.

#[path = "../../capsule_net_nym/src/topology/draw.rs"]
pub mod draw;

#[cfg(test)]
mod draw_tests;
