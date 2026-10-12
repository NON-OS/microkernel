//! One spend proved through the prover. The prover checks the request, verifies its proof and
//! certifies the zero knowledge rank condition. A shortfall, about one proof in 2,044, is proved
//! again with fresh entropy. Refusal text is dropped, since it can name values and leaves.

use super::entropy::SpendEntropy;
use super::request::SpendRequest;
use super::seed::seed_json;
use crate::error::ProveError;
use crate::notes::NotePlaintext;
use crate::prover::Cancel;
use nonos_stark::air::RATE;
use nox_prover::{Error, Options};

#[path = "prove_finish.rs"]
mod prove_finish;
#[path = "prove_proof.rs"]
mod prove_proof;
#[path = "prove_refusal.rs"]
mod prove_refusal;
use prove_finish::finish;
pub use prove_proof::LaunchProof;
pub(crate) use prove_refusal::refusal;

/// Proofs made before a rank failure is reported instead of retried.
const ATTEMPTS: usize = 3;

/// Prove `inputs`, owned under `secret`, against `request`, reusing `cache` when held.
pub fn prove_spend(
    request: &SpendRequest,
    secret: &[u64; RATE],
    inputs: [&NotePlaintext; 2],
    cache: Option<&[u8]>,
    cancel: &Cancel,
) -> Result<LaunchProof, ProveError> {
    let (json, seed) = (request.to_json(), seed_json(secret, inputs));
    let mut cache = cache;
    let report = |phase: nox_prover::Phase, fraction: f32| cancel.report(phase, fraction);
    for _ in 0..ATTEMPTS {
        cancel.restart();
        let entropy = SpendEntropy::draw()?;
        let attempt = match cache {
            Some(top) => {
                let opts = Options {
                    cache: Some(top),
                    progress: Some(&report),
                    cancel: Some(cancel.flag()),
                };
                nox_prover::prove_with(&json, &seed, entropy.consume()?, &opts)
                    .map(|(p, _)| (p, None))
            }
            None => nox_prover::prove_keeping_cache(&json, &seed, entropy.consume()?)
                .map(|(p, c)| (p, Some(c)))
                .map_err(Error::Request),
        };
        // A rank shortfall, about one proof in 2,044, is proved again with fresh entropy.
        let (proof, new_cache) = match attempt {
            Ok(done) => done,
            Err(Error::Rank(_)) => continue,
            // A cache kept from another circuit, the launch one before v2, is dropped and rebuilt.
            Err(Error::Cache(_)) if cache.is_some() => {
                cache = None;
                continue;
            }
            Err(Error::Request(why)) if why.contains("prove again with fresh entropy") => continue,
            Err(e) => return Err(refusal(e)),
        };
        match finish(proof, cache, new_cache)? {
            Some(done) => return Ok(done),
            // A rank shortfall, about one proof in 2,044, is proved again.
            None => continue,
        }
    }
    Err(ProveError::SelfVerify)
}
