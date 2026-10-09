//! The phase profile of a proof on a phone: the pinned `transfer-eth` vector proved
//! from the periodic cache, each phase timed on a monotonic clock. The vector fixes the entropy,
//! so every run makes one proof, and a run whose proof differs says so.

mod marks;
mod pool;
mod report;
mod vector;

#[cfg(test)]
#[path = "profile_test.rs"]
mod profile_test;

pub use report::{PhaseTime, ProofPhase, ProofProfile};

use crate::error::{ProveError, WalletError};
use crate::prover::launch::{cache, prove::refusal};
use crate::prover::Cancel;
use marks::{micros_since, split, Marks};
use nox_prover::{Error, Options, Phase, ENTROPY_BYTES};
use std::path::Path;
use std::time::Instant;

/// Prove the vector on `threads` threads, 0 for every core, from the cache under `dir`.
pub fn profile(dir: &Path, threads: u32, cancel: &Cancel) -> Result<ProofProfile, WalletError> {
    let entropy = vector::entropy().ok_or(ProveError::Encoding)?;
    if cancel.cancelled() {
        return Err(ProveError::Cancelled.into());
    }
    let (top, cache_built_micros) = held_or_built(dir, &entropy)?;
    let marks = Marks::default();
    let mark = |phase: Phase, _: f32| marks.mark(phase);
    let opts = Options { cache: Some(&top), progress: Some(&mark), cancel: Some(cancel.flag()) };
    let ((proved, start, end), threads) = pool::on(threads, || {
        let start = Instant::now();
        let proved = nox_prover::prove_with(vector::REQUEST, vector::SEED, &entropy, &opts);
        (proved, start, Instant::now())
    })?;
    let (proof, _) = proved.map_err(refusal)?;
    let (phases, after_verified_micros, total_micros) =
        split(start, &marks.take(), end).ok_or(ProveError::Encoding)?;
    Ok(ProofProfile {
        phases,
        after_verified_micros,
        total_micros,
        matched: vector::matches(&proof),
        threads,
        cache_built_micros,
    })
}

/// The cache under `dir` if it loads. Otherwise one proof of the vector builds a new one, kept
/// for the next run, since only a proof from the cache reports the phases past Blinding.
fn held_or_built(
    dir: &Path,
    entropy: &[u8; ENTROPY_BYTES],
) -> Result<(Vec<u8>, Option<u64>), WalletError> {
    if let Some(top) = cache::read(dir).filter(|b| nox_prover::load_cache(b).is_ok()) {
        return Ok((top, None));
    }
    let start = Instant::now();
    let (_, built) = nox_prover::prove_keeping_cache(vector::REQUEST, vector::SEED, entropy)
        .map_err(|why| refusal(Error::Request(why)))?;
    cache::keep(dir, &built);
    Ok((built, Some(micros_since(start))))
}
