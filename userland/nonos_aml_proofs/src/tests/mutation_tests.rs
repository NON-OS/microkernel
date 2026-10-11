// NONOS Operating System (AGPL-3.0-or-later)
//! Hostile bytes: thousands of mutated catalogues and random tables. Every
//! one ends in a node tree or a refusal, never a panic, and a refusal leaves
//! the namespace's scope stack where it began.

use super::catalog::laid_out;
use crate::enc::*;
use crate::flat::{parse_with, Flat};

/// xorshift64, so a failure names a seed that reproduces it.
fn next(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

#[test]
fn mutated_catalogues_never_panic() {
    let (aml, _) = laid_out();
    let base = scope("\\_SB", &aml);
    let mut seed = 0x9E37_79B9_7F4A_7C15u64;
    for round in 0..4000 {
        let mut m = base.clone();
        for _ in 0..1 + next(&mut seed) % 4 {
            let at = (next(&mut seed) % m.len() as u64) as usize;
            m[at] = next(&mut seed) as u8;
        }
        let mut flat = Flat::default();
        let _ = parse_with(&m, &mut flat, Default::default());
        assert_eq!(flat.depth, 0, "round {round}");
    }
}

#[test]
fn random_tables_never_panic() {
    let mut seed = 0xD1B5_4A32_D192_ED03u64;
    for round in 0..4000 {
        let len = (next(&mut seed) % 200) as usize;
        let aml: Vec<u8> = (0..len).map(|_| next(&mut seed) as u8).collect();
        let mut flat = Flat::default();
        let _ = parse_with(&aml, &mut flat, Default::default());
        assert_eq!(flat.depth, 0, "round {round}");
    }
}
