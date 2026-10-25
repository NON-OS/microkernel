// NONOS Operating System (AGPL-3.0-or-later)
//! Rounds over the ABI words a deposit read decodes: a bool is 0 or 1 and nothing else, and an
//! amount saturates once its high half is not zero.

use super::abi::{amount, flag};
use crate::fuzz::rng::Rng;

/// A word from the rounds: noise, or mostly zero so the edges are met, not only noise.
fn word(rng: &mut Rng) -> [u8; 32] {
    let mut w = [0u8; 32];
    w.copy_from_slice(&rng.bytes(32));
    if rng.below(2) == 0 {
        let keep = rng.below(33);
        for (i, b) in w.iter_mut().enumerate() {
            if i < keep || rng.below(3) != 0 {
                *b = 0;
            }
        }
    }
    w
}

/// Run `rounds` rounds from `seed`; returns the number run.
pub fn word_rounds(seed: u64, rounds: usize) -> usize {
    let mut rng = Rng::new(seed);
    for _ in 0..rounds {
        let w = word(&mut rng);
        let (high, last) = w.split_at(31);
        let small = high.iter().all(|b| *b == 0);
        match flag(w) {
            Ok(v) => assert!(small && last == [u8::from(v)], "a bool is 0 or 1"),
            Err(_) => assert!(!(small && last[0] <= 1), "0 and 1 are bools"),
        }
        let (top, low) = w.split_at(16);
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(low);
        let want = if top.iter().any(|b| *b != 0) { u128::MAX } else { u128::from_be_bytes(bytes) };
        assert_eq!(amount(w), want, "an amount saturates past u128");
    }
    for last in [0u8, 1, 2, 0xff] {
        let mut w = [0u8; 32];
        w[31] = last;
        assert_eq!(flag(w).is_ok(), last <= 1, "the last byte {last}");
    }
    let mut w = [0u8; 32];
    w[15] = 1;
    assert_eq!(amount(w), u128::MAX, "one bit past u128");
    rounds
}
