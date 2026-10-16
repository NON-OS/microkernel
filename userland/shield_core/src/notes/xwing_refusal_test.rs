//! The blobs a payee's reader turns away: a wrong length or version, another payee's, a moved leaf, a forged blinding.

use super::super::xwing::{open, seal, seal_with, Opened, Opening, BLOB_LEN};
use super::{leaf_of, vector_opening};
use x_wing::{DecapsulationKey, Decapsulator};

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
