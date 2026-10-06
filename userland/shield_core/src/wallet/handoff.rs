//! What the phone hands a relayer: the proof, its limbs, and the two created notes sealed.
//! The relayer submits and is the fee recipient, so the sender's address never touches the chain.
//! Change is sealed to this wallet's own address, so a restore finds it again.

use crate::error::{StoreError, WalletError};
use crate::keys::ENCAPSULATION_KEY_BYTES;
use crate::notes::{commitment, seal_xwing, wire_digest, NotePlaintext, Opening, BLOB_LEN};
use crate::prover::launch::prove::LaunchProof;
use crate::prover::pool_hasher;
use std::path::{Path, PathBuf};

/// Seal both outputs: the payee's to `payee`, the change to `own`.
pub fn seal_outputs(
    proof: &LaunchProof,
    payee: &[u8; ENCAPSULATION_KEY_BYTES],
    own: &[u8; ENCAPSULATION_KEY_BYTES],
) -> Result<[[u8; BLOB_LEN]; 2], WalletError> {
    let [to_payee, change] = &proof.outputs;
    Ok([seal_to(to_payee, payee)?, seal_to(change, own)?])
}

fn seal_to(
    note: &NotePlaintext,
    ek: &[u8; ENCAPSULATION_KEY_BYTES],
) -> Result<[u8; BLOB_LEN], WalletError> {
    let to = x_wing::EncapsulationKey::try_from(ek.as_slice()).map_err(|_| WalletError::Address)?;
    let cm = commitment(&pool_hasher(), &note.note()).map(|f| f.value());
    let opening = Opening { value: note.value, asset_id: note.asset_id, blinding: note.blinding };
    Ok(seal_xwing(&opening, &to, &wire_digest(&cm))?)
}

/// The four files the relayer reads, written into `dir`, emptied first so nothing from an
/// earlier spend goes along.
pub fn write_handoff(
    dir: &Path,
    proof: &LaunchProof,
    sealed: &[[u8; BLOB_LEN]; 2],
) -> Result<Vec<PathBuf>, WalletError> {
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir).map_err(|_| StoreError::Io)?;
    let limbs: Vec<String> = proof.publics.iter().map(u64::to_string).collect();
    let [payee, change] = sealed;
    let files: [(&str, Vec<u8>); 4] = [
        ("spend.proof", proof.bytes.clone()),
        (
            "spend.proof.publics.json",
            format!("{{\"publics\": [{}]}}\n", limbs.join(", ")).into_bytes(),
        ),
        ("blob0.bin", payee.to_vec()),
        ("blob1.bin", change.to_vec()),
    ];
    files
        .iter()
        .map(|(name, body)| {
            let path = dir.join(name);
            std::fs::write(&path, body).map_err(|_| StoreError::Io)?;
            Ok(path)
        })
        .collect()
}

#[cfg(test)]
#[path = "handoff_test.rs"]
mod handoff_test;
