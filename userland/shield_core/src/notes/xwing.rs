//! The note a payee opens, 1,186 bytes: version `0x01`, view tag, the X-Wing ciphertext
//! (ML-KEM-768 then X25519), then ChaCha20-Poly1305 of the 48-byte opening. The nonce is zero
//! because every message has its own key. The note stays sealed while either X25519 or
//! ML-KEM-768 holds, and there is no classical-only version. `xwing_test.rs` holds the pinned
//! vector. The opening must recompute to the emitted leaf, a check no pool can make.

use crate::entropy::fill;
use crate::error::CustodyError;
use crate::notes::{commitment, wire_digest, NotePlaintext};
use crate::prover::pool_hasher;
use chacha20poly1305::{AeadInOut, ChaCha20Poly1305, KeyInit};
use sha3::{Digest, Sha3_256};
use x_wing::{Decapsulate, DecapsulationKey, EncapsulationKey};
use zeroize::{Zeroize, ZeroizeOnDrop};

pub const VERSION: u8 = 0x01;
pub const CIPHERTEXT_LEN: usize = 1120;
pub const OPENING_LEN: usize = 48;
const TAG_LEN: usize = 16;
/// The fixed blob length. The settlement has no room for more.
pub const BLOB_LEN: usize = 2 + CIPHERTEXT_LEN + OPENING_LEN + TAG_LEN;
const SEALED_AT: usize = 2 + CIPHERTEXT_LEN;
const TAG_AT: usize = SEALED_AT + OPENING_LEN;
const VIEW_LABEL: &[u8] = b"NOX-NOTE-VIEW-TAG-v1";
const SEAL_LABEL: &[u8] = b"NOX-NOTE-SEAL-KEY-v1";

/// What a payee needs to spend. The spend key is the payee's own, so it is not carried.
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct Opening {
    pub value: u64,
    pub asset_id: u64,
    pub blinding: [u64; 4],
}

impl Opening {
    fn to_bytes(&self) -> [u8; OPENING_LEN] {
        let mut out = [0u8; OPENING_LEN];
        let words = [
            self.value,
            self.asset_id,
            self.blinding[0],
            self.blinding[1],
            self.blinding[2],
            self.blinding[3],
        ];
        for (slot, w) in out.chunks_exact_mut(8).zip(words.iter()) {
            slot.copy_from_slice(&w.to_le_bytes());
        }
        out
    }

    fn from_bytes(b: &[u8; OPENING_LEN]) -> Opening {
        let mut words = [0u64; 6];
        for (w, chunk) in words.iter_mut().zip(b.chunks_exact(8)) {
            let mut le = [0u8; 8];
            le.copy_from_slice(chunk);
            *w = u64::from_le_bytes(le);
        }
        Opening {
            value: words[0],
            asset_id: words[1],
            blinding: [words[2], words[3], words[4], words[5]],
        }
    }
}

/// How far a blob got before it was ruled out, in the order the reader tries.
pub enum Opened {
    /// Not this version's length or version, or the view tag says not ours.
    Skip,
    /// The tag matched and the seal did not open: a one-in-256 collision.
    Refuse,
    /// It opened and does not recompute to the leaf: not a note for this key.
    Discard,
    Note(Opening),
}

fn sha3(label: &[u8], ss: &[u8]) -> [u8; 32] {
    Sha3_256::new().chain_update(label).chain_update(ss).finalize().into()
}

fn aad(tag: u8, leaf: &[u8; 32]) -> [u8; 34] {
    let mut out = [0u8; 34];
    out[0] = VERSION;
    out[1] = tag;
    out[2..].copy_from_slice(leaf);
    out
}

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

/// Read a blob as this payee's note, in the five steps and never out of order.
pub fn open(blob: &[u8], dk: &DecapsulationKey, leaf: &[u8; 32], spend_pk: &[u64; 4]) -> Opened {
    // 1. A length or version this reader does not know is skipped, not guessed.
    if blob.len() != BLOB_LEN || blob.first() != Some(&VERSION) {
        return Opened::Skip;
    }
    let (Some(&tag), Some(ct), Some(sealed), Some(sealed_tag)) = (
        blob.get(1),
        blob.get(2..SEALED_AT),
        blob.get(SEALED_AT..TAG_AT),
        blob.get(TAG_AT..BLOB_LEN),
    ) else {
        return Opened::Skip;
    };
    let Ok(ct) = ct.try_into() else { return Opened::Skip };
    // 2. Decapsulate and derive the tag. A mismatch is almost certainly not ours.
    let ss = dk.decapsulate(&ct);
    if sha3(VIEW_LABEL, &ss)[0] != tag {
        return Opened::Skip;
    }
    // 3. Open the seal with the leaf in the associated data.
    let mut key = sha3(SEAL_LABEL, &ss);
    let cipher = ChaCha20Poly1305::new(&key.into());
    key.zeroize();
    let mut body = [0u8; OPENING_LEN];
    body.copy_from_slice(sealed);
    let Ok(sealed_tag) = sealed_tag.try_into() else { return Opened::Skip };
    if cipher
        .decrypt_inout_detached(
            &[0u8; 12].into(),
            &aad(tag, leaf),
            body.as_mut_slice().into(),
            &sealed_tag,
        )
        .is_err()
    {
        return Opened::Refuse;
    }
    let opening = Opening::from_bytes(&body);
    body.zeroize();
    // A non-canonical blinding word commits like its canonical form, so it would pass
    // the leaf check as a secret no prover can witness. Refused, as the reference does.
    if opening.blinding.iter().any(|w| *w >= nonos_stark::field::P) {
        return Opened::Discard;
    }
    // Over MAX_VALUE or a non-field asset id: a note the pool never made.
    if opening.value > super::MAX_VALUE || opening.asset_id >= nonos_stark::field::P {
        return Opened::Discard;
    }
    // 4 and 5. A blob that does not recompute to the leaf is not a note, whatever it says.
    let plain = NotePlaintext {
        value: opening.value,
        asset_id: opening.asset_id,
        blinding: opening.blinding,
        spend_pk: *spend_pk,
    };
    let cm = commitment(&pool_hasher(), &plain.note());
    if wire_digest(&[cm[0].value(), cm[1].value(), cm[2].value(), cm[3].value()]) != *leaf {
        return Opened::Discard;
    }
    Opened::Note(opening)
}
