/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used, clippy::indexing_slicing)]

use super::xwing::{open, seal, seal_with, Opened, Opening, BLOB_LEN};
use crate::notes::{commitment, wire_digest, NotePlaintext};
use crate::prover::pool_hasher;
use sha3::{Digest, Sha3_256};
use x_wing::{DecapsulationKey, Decapsulator, KeyExport};

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn vector_opening() -> Opening {
    Opening { value: 1_000_000_000_000_000_000, asset_id: 1, blinding: [11, 22, 33, 44] }
}

/// The leaf a real note commits to, for a payee's spend key.
fn leaf_of(o: &Opening, spend_pk: &[u64; 4]) -> [u8; 32] {
    let plain = NotePlaintext {
        value: o.value,
        asset_id: o.asset_id,
        blinding: o.blinding,
        spend_pk: *spend_pk,
    };
    let cm = commitment(&pool_hasher(), &plain.note());
    wire_digest(&[cm[0].value(), cm[1].value(), cm[2].value(), cm[3].value()])
}

/*
 * The prover's pinned vector, through the production seal. Reproducing all
 * three is what it means for two implementations to interoperate.
 */
#[test]
fn the_pinned_vector_is_reproduced() {
    let dk = DecapsulationKey::from([0x07u8; 32]);
    let ek = dk.encapsulation_key();
    let ek_sha3: [u8; 32] = Sha3_256::digest(ek.to_bytes()).into();
    assert_eq!(hex(&ek_sha3), "4c82604001ad0b7a4cbc022c322f5e757f09ba4a5b095c8af545caeeac6836c8");

    let blob = seal_with(&vector_opening(), ek, &[0x5a; 32], &[0x09; 64]).expect("seal");
    assert_eq!(blob.len(), 1186);
    assert_eq!(blob[1], 0xd2, "view tag");
    let blob_sha3: [u8; 32] = Sha3_256::digest(blob).into();
    assert_eq!(hex(&blob_sha3), "78fbb6f16dfdcf4b5cc87ecb0cca92c4aa44b4747f3b2c183a9d3eb9152e1dc9");
}

#[test]
fn a_real_note_round_trips() {
    let dk = DecapsulationKey::from([0x21u8; 32]);
    let spend_pk = [5, 6, 7, 8];
    let o = vector_opening();
    let leaf = leaf_of(&o, &spend_pk);
    let blob = seal(&o, dk.encapsulation_key(), &leaf).expect("seal");
    match open(&blob, &dk, &leaf, &spend_pk) {
        Opened::Note(got) => {
            assert_eq!((got.value, got.asset_id, got.blinding), (o.value, o.asset_id, o.blinding))
        }
        _ => panic!("a note sealed to us does not open"),
    }
}

#[test]
fn step_five_discards_a_blob_that_does_not_recompute_to_its_leaf() {
    // The pinned vector sits beside a leaf no real note commits to. The seal
    // opens, and the note is still refused as money.
    let dk = DecapsulationKey::from([0x07u8; 32]);
    let blob = seal_with(&vector_opening(), dk.encapsulation_key(), &[0x5a; 32], &[0x09; 64])
        .expect("seal");
    assert!(matches!(open(&blob, &dk, &[0x5a; 32], &[1, 2, 3, 4]), Opened::Discard));
}

#[test]
fn a_wrong_length_or_version_is_skipped_unread() {
    let dk = DecapsulationKey::from([0x07u8; 32]);
    let blob = seal_with(&vector_opening(), dk.encapsulation_key(), &[0x5a; 32], &[0x09; 64])
        .expect("seal");
    assert!(matches!(open(&blob[..BLOB_LEN - 1], &dk, &[0x5a; 32], &[0; 4]), Opened::Skip));
    let mut v2 = blob;
    v2[0] = 0x02;
    assert!(matches!(open(&v2, &dk, &[0x5a; 32], &[0; 4]), Opened::Skip));
}

#[test]
fn a_note_for_another_payee_is_skipped() {
    let ours = DecapsulationKey::from([0x07u8; 32]);
    let theirs = DecapsulationKey::from([0x08u8; 32]);
    let blob = seal_with(&vector_opening(), theirs.encapsulation_key(), &[0x5a; 32], &[0x09; 64])
        .expect("seal");
    // The tag differs in 255 of 256 cases, and for this vector it does.
    assert!(matches!(open(&blob, &ours, &[0x5a; 32], &[0; 4]), Opened::Skip | Opened::Refuse));
}

#[test]
fn a_blob_moved_to_another_leaf_is_refused() {
    // The leaf is in the associated data, so a blob replayed beside a different
    // leaf fails the seal even though its tag still matches.
    let dk = DecapsulationKey::from([0x07u8; 32]);
    let blob = seal_with(&vector_opening(), dk.encapsulation_key(), &[0x5a; 32], &[0x09; 64])
        .expect("seal");
    assert!(matches!(open(&blob, &dk, &[0x5b; 32], &[0; 4]), Opened::Refuse));
}

#[test]
fn a_non_canonical_blinding_is_discarded_even_when_it_opens() {
    // P itself reduces to zero, so this opening commits to the same leaf as a
    // blinding of [0, 2, 3, 4]. It seals and opens, and must still not be a note.
    let dk = DecapsulationKey::from([0x31u8; 32]);
    let spend_pk = [5, 6, 7, 8];
    let honest = Opening { value: 9, asset_id: 1, blinding: [0, 2, 3, 4] };
    let leaf = leaf_of(&honest, &spend_pk);
    let forged = Opening { value: 9, asset_id: 1, blinding: [nonos_stark::field::P, 2, 3, 4] };
    let blob = seal(&forged, dk.encapsulation_key(), &leaf).expect("seal");
    assert!(matches!(open(&blob, &dk, &leaf, &spend_pk), Opened::Discard));
}
