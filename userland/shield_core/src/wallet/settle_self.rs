//! The settlement of a spend by its owner, from the four files the spend wrote: the proof in the
//! verifier's single-call layout, the twelve words of its intent, and the two sealed notes. It is
//! what a lander would send, so the owner can land the spend when no one else does.

use super::one_call::one_call;
use super::relay_body::{limbs, read};
use super::settle::settle_calldata;
use crate::error::{StoreError, WalletError};
use crate::notes::BLOB_LEN;
use crate::prover::launch::publics::{pool_words, LIMBS};
use std::path::Path;

/// The `settleBatch` calldata for the hand-off in `dir`.
pub(crate) fn settle_self(dir: &Path) -> Result<Vec<u8>, WalletError> {
    let shape = || WalletError::from(StoreError::RowShape);
    let proof = one_call(&read(dir, "spend.proof")?).ok_or_else(shape)?;
    let (payee, change) = (read(dir, "blob0.bin")?, read(dir, "blob1.bin")?);
    if payee.len() != BLOB_LEN || change.len() != BLOB_LEN {
        return Err(shape());
    }
    let publics = String::from_utf8(read(dir, "spend.proof.publics.json")?).map_err(|_| shape())?;
    let limbs: [u64; LIMBS] = limbs(&publics).and_then(|l| l.try_into().ok()).ok_or_else(shape)?;
    Ok(settle_calldata(&proof, &pool_words(&limbs), &[payee, change]))
}
