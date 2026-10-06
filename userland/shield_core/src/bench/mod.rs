//! The measurement the product is gated on: a proof at the transfer point, timed on a real
//! phone with peak memory. It builds its own witness so a fresh install can run it.

mod card;
#[cfg(test)]
#[path = "card_test.rs"]
mod card_test;
mod launch;
mod memory;
mod profile;
mod report;

pub use card::card;
pub use launch::bench_launch;
pub use profile::{profile, PhaseTime, ProofPhase, ProofProfile};
pub use report::{BenchReport, Shape};
