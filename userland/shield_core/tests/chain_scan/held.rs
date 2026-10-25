//! A note this wallet held earlier, now spent on chain: its record, words and nullifier.

use super::{leaf_of, EV};
use nox_shield_core::keys::{note_nullifier_wire, Account};
use nox_shield_core::notes::{NotePlaintext, NoteRecord, NoteStatus};

/// The held note's record, its commitment as four words, and its nullifier event's topics.
pub fn held_and_spent(me: &Account) -> ([NoteRecord; 1], [u64; 4], [[u8; 32]; 2]) {
    let held_plain = NotePlaintext {
        value: 42,
        asset_id: 1,
        blinding: [1, 1, 1, 1],
        spend_pk: me.address().spend_pk,
    };
    let held_cm = leaf_of(&held_plain);
    let mut le = held_cm;
    le.reverse();
    let mut words = [0u64; 4];
    for (w, c) in words.iter_mut().zip(le.chunks_exact(8)) {
        *w = u64::from_le_bytes(c.try_into().unwrap());
    }
    let held = [NoteRecord {
        plain: held_plain,
        leaf_index: 9,
        cm: words,
        status: NoteStatus::Unspent,
        found_at: 9,
    }];
    (held, words, [EV, note_nullifier_wire(me, &words, 9)])
}
