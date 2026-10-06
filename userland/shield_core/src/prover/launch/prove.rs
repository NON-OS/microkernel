//! One spend proved through the prover. The prover checks the request, verifies its proof and
//! certifies the zero knowledge rank condition. A shortfall, about one proof in 2,044, is proved
//! again with fresh entropy. Refusal text is dropped, since it can name values and leaves.

use super::entropy::SpendEntropy;
use super::publics::{pool_words, LIMBS, WORDS};
use super::request::SpendRequest;
use super::seed::seed_json;
use crate::error::ProveError;
use crate::notes::NotePlaintext;
use crate::prover::Cancel;
use nonos_stark::air::RATE;
use nox_prover::{Error, Options};

/// Proofs made before a rank failure is reported instead of retried.
const ATTEMPTS: usize = 3;

/// A proved spend and everything settlement needs from it.
pub struct LaunchProof {
    pub bytes: Vec<u8>,
    pub publics: [u64; LIMBS],
    pub words: [[u8; 32]; WORDS],
    /// The two notes created, payee first, with the blindings drawn for them.
    pub outputs: [NotePlaintext; 2],
    pub grind_hashes: u64,
    pub weakened: Vec<&'static str>,
    /// The periodic cache, when this proof built it, for the caller to keep.
    pub new_cache: Option<Vec<u8>>,
}

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
        let publics =
            <[u64; LIMBS]>::try_from(proof.publics.as_slice()).map_err(|_| ProveError::Encoding)?;
        if nox_prover::zk_fri_rank_check(&proof.bytes, &publics).is_err() {
            continue;
        }
        // The pool verifies the shared form, each path once, against the periodic tree in hand.
        let tree = cache.or(new_cache.as_deref()).ok_or(ProveError::Encoding)?;
        let shared = nox_prover::to_shared(&proof.bytes, &publics, tree).map_err(refusal)?;
        return Ok(LaunchProof {
            words: pool_words(&publics),
            outputs: proof.created.notes.map(|n| NotePlaintext {
                value: n.value,
                asset_id: n.asset_id,
                blinding: n.blinding,
                spend_pk: n.spend_pk,
            }),
            bytes: shared,
            publics,
            grind_hashes: proof.grind_hashes,
            weakened: proof.weakened,
            new_cache,
        });
    }
    Err(ProveError::SelfVerify)
}

/// The prover's refusal as the wallet's typed error.
pub(crate) fn refusal(e: Error) -> ProveError {
    match e {
        Error::Cancelled => ProveError::Cancelled,
        Error::Entropy(_) => ProveError::SeedEntropy,
        Error::NotVerified(_) => ProveError::SelfVerify,
        Error::Request(why) if why.contains("does not balance") => ProveError::Unbalanced,
        Error::Request(why)
            if why.contains("rebuild to") || why.contains("is not the pool's leaf") =>
        {
            ProveError::RootNotPublished
        }
        _ => ProveError::WitnessUnsatisfied,
    }
}
