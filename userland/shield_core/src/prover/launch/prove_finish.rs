//! A proof as it came from the prover, checked for the zero knowledge rank,
//! put in the pool's shared form, and its public words built.

use super::super::publics::{pool_words, LIMBS};
use super::{refusal, LaunchProof};
use crate::error::ProveError;
use crate::notes::NotePlaintext;

/// The launch proof, or None for a rank shortfall, which is proved again.
pub(super) fn finish(
    proof: nox_prover::Proof,
    cache: Option<&[u8]>,
    new_cache: Option<Vec<u8>>,
) -> Result<Option<LaunchProof>, ProveError> {
    let publics =
        <[u64; LIMBS]>::try_from(proof.publics.as_slice()).map_err(|_| ProveError::Encoding)?;
    if nox_prover::zk_fri_rank_check(&proof.bytes, &publics).is_err() {
        return Ok(None);
    }
    // The pool verifies the shared form, each path once, against the periodic tree in hand.
    let tree = cache.or(new_cache.as_deref()).ok_or(ProveError::Encoding)?;
    let shared = nox_prover::to_shared(&proof.bytes, &publics, tree).map_err(refusal)?;
    Ok(Some(LaunchProof {
        words: pool_words(&publics)?,
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
    }))
}
