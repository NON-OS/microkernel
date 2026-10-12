/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::indexing_slicing, clippy::unwrap_used, clippy::arithmetic_side_effects)]

use super::{address_limbs, pool_words};

#[path = "publics_vectors.rs"]
mod publics_vectors;
use publics_vectors::WITHDRAW_A;

/// The pool's own expansion, `PublicWords.publicsOf` on a thirteen word intent.
fn expand(words: &[[u8; 32]; 13]) -> Vec<u64> {
    let limb =
        |w: &[u8; 32], from: usize| u64::from_be_bytes(w[from..from + 8].try_into().unwrap());
    let mut out = Vec::new();
    for (i, w) in words.iter().enumerate() {
        if (10..=11).contains(&i) {
            let mut wide = [0u8; 16];
            wide.copy_from_slice(&w[16..]);
            let low = u128::from_be_bytes(wide);
            let high = u128::from(u32::from_be_bytes(w[12..16].try_into().unwrap()));
            let n = |v: u128| u64::try_from(v & ((1 << 48) - 1)).unwrap();
            out.extend([
                n(low),
                n(low >> 48),
                n((low >> 96) | (high << 32)),
                u64::try_from(high >> 16).unwrap(),
            ]);
        } else if i <= 5 {
            out.extend([limb(w, 24), limb(w, 16), limb(w, 8), limb(w, 0)]);
        } else {
            assert!(w[..24].iter().all(|b| *b == 0));
            out.push(limb(w, 24));
        }
    }
    out
}

#[test]
fn the_words_expand_back_to_the_pinned_proofs_statement() {
    assert_eq!(expand(&pool_words(&WITHDRAW_A).unwrap()), WITHDRAW_A.to_vec());
}

#[test]
fn address_limbs_match_the_pinned_withdrawal() {
    let recipient: [u8; 20] = [
        0x8b, 0xa1, 0xf1, 0x09, 0x55, 0x1b, 0xd4, 0x32, 0x80, 0x30, 0x12, 0x64, 0x5a, 0xc1, 0x36,
        0xdd, 0xd6, 0x4d, 0xba, 0x72,
    ];
    assert_eq!(
        address_limbs(&recipient),
        [WITHDRAW_A[28], WITHDRAW_A[29], WITHDRAW_A[30], WITHDRAW_A[31]]
    );
    let mut submitter = [0u8; 20];
    submitter[19] = 1;
    assert_eq!(
        address_limbs(&submitter),
        [WITHDRAW_A[32], WITHDRAW_A[33], WITHDRAW_A[34], WITHDRAW_A[35]]
    );
    assert_eq!(address_limbs(&[0xff; 20]), [(1 << 48) - 1, (1 << 48) - 1, (1 << 48) - 1, 0xffff]);
}

#[path = "publics_bound_test.rs"]
mod publics_bound_test;
