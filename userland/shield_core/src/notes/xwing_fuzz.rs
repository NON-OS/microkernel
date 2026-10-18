// NONOS Operating System (AGPL-3.0-or-later)
//! Rounds over a note's open: every length about BLOB_LEN skipped, a sealed note opened, and no
//! flipped bit or noise ever opened as a note.

use super::sealing::seal_with;
use super::{open, Opened, Opening, BLOB_LEN, VERSION};
use crate::fuzz::rng::Rng;
use crate::notes::{commitment, wire_digest, NotePlaintext};
use crate::prover::pool_hasher;
use x_wing::{DecapsulationKey, Decapsulator};

/// The leaf a real note commits to, for this opening and spend key.
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

/// Run `rounds` rounds from `seed`; returns the number run.
pub fn note_rounds(seed: u64, rounds: usize) -> usize {
    let mut rng = Rng::new(seed);
    let dk = DecapsulationKey::from([0x07u8; 32]);
    let spend = [3, 1, 4, 1];
    let opening = Opening { value: rng.next(), asset_id: 1, blinding: [rng.next(), 2, 3, 4] };
    let leaf = leaf_of(&opening, &spend);
    let mut randomness = [0u8; 64];
    randomness.copy_from_slice(&rng.bytes(64));
    let sealed = seal_with(&opening, dk.encapsulation_key(), &leaf, &randomness).expect("seal");
    match open(&sealed, &dk, &leaf, &spend) {
        Opened::Note(o) => assert!(o.value == opening.value && o.blinding == opening.blinding),
        _ => panic!("a note sealed to this key and leaf opens"),
    }
    // Only BLOB_LEN is read: one byte short, one past, none, and far past are skipped.
    for len in [0, 1, 2, BLOB_LEN - 1, BLOB_LEN + 1, 2 * BLOB_LEN] {
        let mut blob = rng.bytes(len);
        if let Some(first) = blob.first_mut() {
            *first = VERSION;
        }
        assert!(matches!(open(&blob, &dk, &leaf, &spend), Opened::Skip), "{len} bytes");
    }
    for _ in 0..rounds {
        let mut blob = sealed;
        if let Some(byte) = blob.get_mut(rng.below(BLOB_LEN)) {
            *byte ^= 1 << rng.below(8);
        }
        assert!(!matches!(open(&blob, &dk, &leaf, &spend), Opened::Note(_)), "a flipped bit");
        let noise = rng.bytes(BLOB_LEN);
        assert!(!matches!(open(&noise, &dk, &leaf, &spend), Opened::Note(_)), "noise");
    }
    rounds
}
