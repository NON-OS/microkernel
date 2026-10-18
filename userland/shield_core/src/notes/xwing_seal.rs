//! Sealing an opening to a payee's encapsulation key, bound to its leaf.

use super::{
    aad, sha3, Opening, BLOB_LEN, CIPHERTEXT_LEN, OPENING_LEN, SEAL_LABEL, VERSION, VIEW_LABEL,
};
use crate::entropy::fill;
use crate::error::CustodyError;
use chacha20poly1305::{AeadInOut, ChaCha20Poly1305, KeyInit};
use x_wing::EncapsulationKey;
use zeroize::Zeroize;

/// Seal an opening to a payee's encapsulation key, bound to its leaf. The 64 bytes of
/// encapsulation randomness come fresh from the platform CSPRNG for every note.
pub fn seal(
    opening: &Opening,
    to: &EncapsulationKey,
    leaf: &[u8; 32],
) -> Result<[u8; BLOB_LEN], CustodyError> {
    let mut randomness = [0u8; 64];
    fill(&mut randomness)?;
    let out = seal_with(opening, to, leaf, &randomness);
    randomness.zeroize();
    out
}

/// The seal with supplied randomness, to reproduce the pinned vector.
pub(crate) fn seal_with(
    opening: &Opening,
    to: &EncapsulationKey,
    leaf: &[u8; 32],
    randomness: &[u8; 64],
) -> Result<[u8; BLOB_LEN], CustodyError> {
    let (ct, ss) = to.encapsulate_deterministic(&(*randomness).into());
    let tag = sha3(VIEW_LABEL, &ss)[0];
    let mut key = sha3(SEAL_LABEL, &ss);
    let cipher = ChaCha20Poly1305::new(&key.into());
    key.zeroize();
    let mut body = opening.to_bytes();
    let sealed_tag = cipher
        .encrypt_inout_detached(&[0u8; 12].into(), &aad(tag, leaf), body.as_mut_slice().into())
        .map_err(|_| CustodyError::SealAuth)?;

    let mut blob = [0u8; BLOB_LEN];
    let (head, rest) = blob.split_at_mut(2);
    head.copy_from_slice(&[VERSION, tag]);
    let (ct_slot, rest) = rest.split_at_mut(CIPHERTEXT_LEN);
    ct_slot.copy_from_slice(&ct);
    let (body_slot, tag_slot) = rest.split_at_mut(OPENING_LEN);
    body_slot.copy_from_slice(&body);
    tag_slot.copy_from_slice(&sealed_tag);
    body.zeroize();
    Ok(blob)
}
