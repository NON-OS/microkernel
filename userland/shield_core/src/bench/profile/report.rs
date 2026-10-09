//! What one profiled proof returns to the shell. Peak memory is not here: each app reads it from
//! its own platform, which the core cannot do the same way on both.

use nox_prover::Phase;

/// The eleven phases the prover reports, in the order it reports them.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProofPhase {
    Request,
    Blinding,
    Region,
    Products,
    Periodic,
    Composition,
    CompositionTree,
    Deep,
    Fri,
    Queries,
    Verified,
}

impl From<Phase> for ProofPhase {
    fn from(phase: Phase) -> ProofPhase {
        match phase {
            Phase::Request => ProofPhase::Request,
            Phase::Blinding => ProofPhase::Blinding,
            Phase::Region => ProofPhase::Region,
            Phase::Products => ProofPhase::Products,
            Phase::Periodic => ProofPhase::Periodic,
            Phase::Composition => ProofPhase::Composition,
            Phase::CompositionTree => ProofPhase::CompositionTree,
            Phase::Deep => ProofPhase::Deep,
            Phase::Fri => ProofPhase::Fri,
            Phase::Queries => ProofPhase::Queries,
            Phase::Verified => ProofPhase::Verified,
        }
    }
}

/// One phase and the microseconds from the end of the phase before it to its own end.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PhaseTime {
    pub phase: ProofPhase,
    pub micros: u64,
}

/// One proof of the pinned vector, timed on a monotonic clock.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProofProfile {
    pub phases: Vec<PhaseTime>,
    /// From Verified to the return: the prover encodes the proof and checks its rank certificate
    /// there, and reports no phase for it. The phases and this add up to the total.
    pub after_verified_micros: u64,
    pub total_micros: u64,
    /// Whether the proof, written as the vector writes it, is `proof.json` byte for byte.
    pub matched: bool,
    pub threads: u32,
    /// Set when no periodic cache was held: this call first proved once to build it, in this time.
    pub cache_built_micros: Option<u64>,
}
