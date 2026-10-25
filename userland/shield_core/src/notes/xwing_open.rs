//! Reading a blob as this payee's note, in the five steps and never out of order.

use super::{aad, sha3, Opened, Opening, BLOB_LEN, OPENING_LEN, SEALED_AT, SEAL_LABEL, TAG_AT};
use super::{VERSION, VIEW_LABEL};
use crate::notes::{commitment, wire_digest, NotePlaintext};
use crate::prover::pool_hasher;
use chacha20poly1305::{AeadInOut, ChaCha20Poly1305, KeyInit};
use x_wing::{Decapsulate, DecapsulationKey};
use zeroize::Zeroize;

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
    if opening.value > super::super::MAX_VALUE || opening.asset_id >= nonos_stark::field::P {
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
