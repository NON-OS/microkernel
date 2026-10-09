//! The instants the prover reports its phases at, and the times between them.

use super::report::{PhaseTime, ProofPhase};
use nox_prover::Phase;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Each phase as the prover reports it, with the instant it arrived.
#[derive(Default)]
pub struct Marks {
    seen: Mutex<Vec<(ProofPhase, Instant)>>,
}

impl Marks {
    /// The progress callback: take the clock first, then the lock.
    pub fn mark(&self, phase: Phase) {
        let at = Instant::now();
        if let Ok(mut seen) = self.seen.lock() {
            seen.push((phase.into(), at));
        }
    }

    pub fn take(self) -> Vec<(ProofPhase, Instant)> {
        self.seen.into_inner().unwrap_or_default()
    }
}

/// The time of each phase, the time after the last one, and the total, all from `start` to
/// `end`. Nothing when a mark lies outside them or before the mark ahead of it.
pub fn split(
    start: Instant,
    marks: &[(ProofPhase, Instant)],
    end: Instant,
) -> Option<(Vec<PhaseTime>, u64, u64)> {
    let mut last = start;
    let mut phases = Vec::with_capacity(marks.len());
    for &(phase, at) in marks {
        phases.push(PhaseTime { phase, micros: micros(at.checked_duration_since(last)?) });
        last = at;
    }
    let after = micros(end.checked_duration_since(last)?);
    Some((phases, after, micros(end.checked_duration_since(start)?)))
}

/// The microseconds since `start`.
pub fn micros_since(start: Instant) -> u64 {
    micros(start.elapsed())
}

fn micros(span: Duration) -> u64 {
    u64::try_from(span.as_micros()).unwrap_or(u64::MAX)
}
