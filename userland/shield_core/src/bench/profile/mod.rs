//! The phase profile of a proof on a phone: the pinned `transfer-eth` vector proved
//! from the periodic cache, each phase timed on a monotonic clock. The vector fixes the entropy,
//! so every run makes one proof, and a run whose proof differs says so.

mod marks;
mod pool;
mod report;
mod run;
mod vector;

#[cfg(test)]
#[path = "profile_test.rs"]
mod profile_test;

pub use report::{PhaseTime, ProofPhase, ProofProfile};
pub use run::profile;
