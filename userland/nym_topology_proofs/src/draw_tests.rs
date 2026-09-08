// NONOS Operating System (AGPL-3.0-or-later)
//! What a route draw has to satisfy before it is allowed to pick a hop.
//!
//! The counts used here are the live active sets: 20 nodes in a mix layer,
//! 180 entry gateways, 179 exits. The old draw was one seed byte modulo the
//! count, and these are the properties it failed.

use crate::draw::draw;

/// The active sets, per role, as the directory endpoints answer today.
const MIX_LAYER: usize = 20;
const ENTRY: usize = 180;
const EXIT: usize = 179;

/// A seed whose bytes are all distinct, so a draw that reads the wrong four
/// of them produces a different answer rather than the same one by accident.
fn seed(tag: u8) -> [u8; 32] {
    let mut s = [0u8; 32];
    for (i, b) in s.iter_mut().enumerate() {
        *b = tag.wrapping_mul(31).wrapping_add(i as u8).wrapping_mul(7);
    }
    s
}

#[test]
fn stays_in_range_for_every_salt_and_count() {
    for len in [1usize, 2, 3, MIX_LAYER, EXIT, ENTRY, 255, 256, 257, 512] {
        for salt in 0u8..=255 {
            for tag in 0u8..=64 {
                assert!(draw(&seed(tag), salt, len) < len, "len {len} salt {salt}");
            }
        }
    }
}

/// The salt is masked rather than trusted, so no caller can index past the
/// seed. A hop that panics is a route that never leaves.
#[test]
fn no_salt_can_read_past_the_seed() {
    for salt in 0u8..=255 {
        let _ = draw(&seed(1), salt, ENTRY);
    }
}

#[test]
fn empty_candidate_set_does_not_panic() {
    assert_eq!(draw(&seed(3), 1, 0), 0);
}

#[test]
fn a_single_candidate_is_always_chosen() {
    for salt in 0u8..=7 {
        assert_eq!(draw(&seed(salt), salt, 1), 0);
    }
}

#[test]
fn the_same_seed_draws_the_same_hop() {
    let s = seed(9);
    for salt in 1u8..=4 {
        assert_eq!(draw(&s, salt, ENTRY), draw(&s, salt, ENTRY));
    }
}

/// Different hops of one route must not collapse onto the same index, or a
/// route would repeat a node across layers whenever the layers matched.
#[test]
fn different_salts_read_different_seed_bytes() {
    let s = seed(11);
    let picks: Vec<usize> = (1u8..=4).map(|salt| draw(&s, salt, ENTRY)).collect();
    assert!(picks.iter().collect::<std::collections::HashSet<_>>().len() > 1);
}

/// The whole list has to be reachable. One byte modulo the count could never
/// name an index above 255 however long the list was, so on a 512-node store
/// the tail was unreachable by construction.
#[test]
fn every_index_is_reachable_on_a_long_list() {
    let len = 512;
    let mut seen = vec![false; len];
    // Walk the four seed bytes the draw reads as one independent counter.
    // Deriving them from each other would leave combinations unvisited and
    // prove nothing about the draw.
    for hi in 0u32..=0xffff {
        let x = hi << 16;
        let mut s = [0u8; 32];
        s[4..8].copy_from_slice(&x.to_le_bytes());
        seen[draw(&s, 1, len)] = true;
    }
    let missed = seen.iter().filter(|hit| !**hit).count();
    assert_eq!(missed, 0, "{missed} of {len} candidates unreachable");
}

/// No candidate may be favoured. Over the whole draw range each of `len`
/// candidates should take a `1/len` share; the multiply-shift reduction is
/// exact to within one draw, so the observed counts differ by at most one.
fn share_is_even(len: usize) {
    let mut counts = vec![0u64; len];
    // Walk the top bits of the draw range, which is what the reduction reads.
    for hi in 0u32..=0xffff {
        let x = hi << 16;
        let idx = ((x as u64 * len as u64) >> 32) as usize;
        counts[idx] += 1;
    }
    let lo = *counts.iter().min().unwrap();
    let hi = *counts.iter().max().unwrap();
    assert!(hi - lo <= 1, "len {len}: share ranged {lo}..={hi}");
}

#[test]
fn no_candidate_is_favoured_in_a_mix_layer() {
    share_is_even(MIX_LAYER);
}

#[test]
fn no_candidate_is_favoured_across_the_gateway_sets() {
    share_is_even(ENTRY);
    share_is_even(EXIT);
}

/// The regression this replaced. A byte modulo 180 gives the first 76
/// candidates two chances and the rest one, a 2:1 lean toward the front of
/// the list that an observer can measure and the client cannot see. Recorded
/// here so the old shape cannot come back quietly.
#[test]
fn the_modulo_draw_it_replaced_was_biased() {
    let len = ENTRY;
    let mut counts = vec![0u64; len];
    for byte in 0u32..256 {
        counts[byte as usize % len] += 1;
    }
    let lo = *counts.iter().min().unwrap();
    let hi = *counts.iter().max().unwrap();
    assert_eq!((lo, hi), (1, 2));
}
