// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

/*
 * Every byte class of a trailer, altered. A valid trailer is checked first in
 * each case, so a refusal is never an artefact of a fixture that did not verify
 * to begin with.
 */

use super::fixture::{capsule_ctx, enroll, policy, DEPTH, EPOCH};
use crate::leaf::Kind;
use crate::verify::verify;

fn good() -> ([u8; 32], Vec<u8>, Vec<u8>) {
    let (root, tree, cap, _) = policy();
    let t = tree.trailer(0).expect("trailer");
    let ctx = capsule_ctx(&cap, 0x7, EPOCH);
    assert!(verify(&root, DEPTH, Kind::Capsule, &ctx, &t), "fixture must verify");
    (root, t, ctx)
}

fn refused(root: &[u8; 32], ctx: &[u8], t: &[u8]) -> bool {
    !verify(root, DEPTH, Kind::Capsule, ctx, t)
}

#[test]
fn any_changed_magic_byte_is_refused() {
    let (root, t, ctx) = good();
    for i in 0..8 {
        let mut b = t.clone();
        b[i] ^= 0x01;
        assert!(refused(&root, &ctx, &b), "magic byte {i}");
    }
    let mut stark = t.clone();
    stark[..8].copy_from_slice(b"NZKSTRK2");
    assert!(refused(&root, &ctx, &stark), "a STARK trailer is not read as a path");
}

#[test]
fn a_changed_depth_byte_is_refused() {
    let (root, t, ctx) = good();
    for d in [0u8, 7, 9, 32, 33, 255] {
        let mut b = t.clone();
        b[8] = d;
        assert!(refused(&root, &ctx, &b), "depth {d}");
    }
}

/// Each sibling byte, with the flipped bit rotating so every position is hit.
#[test]
fn every_sibling_byte_matters() {
    let (root, t, ctx) = good();
    for i in 9..9 + DEPTH * 32 {
        let mut b = t.clone();
        b[i] ^= 1 << (i % 8);
        assert!(refused(&root, &ctx, &b), "sibling byte {i}");
    }
}

#[test]
fn every_direction_bit_matters() {
    let (root, t, ctx) = good();
    let at = 9 + DEPTH * 32;
    for bit in 0..8 {
        let mut b = t.clone();
        b[at] ^= 1 << bit;
        assert!(refused(&root, &ctx, &b), "direction bit {bit}");
    }
}

#[test]
fn trailing_and_missing_bytes_are_refused() {
    let (root, t, ctx) = good();
    let mut longer = t.clone();
    longer.push(0);
    assert!(refused(&root, &ctx, &longer));
    for cut in 0..t.len() {
        assert!(refused(&root, &ctx, &t[..cut]), "cut at {cut}");
    }
}

/*
 * One value, two encodings: v and v + p name the same element when reduced. The
 * decoder must refuse the second, or a trailer has a twin that verifies. Only a
 * small v has a second encoding inside 64 bits, and no real sibling is that
 * small, so the rule is checked where it lives, on the decoder.
 */
#[test]
fn a_second_encoding_of_the_same_word_is_refused() {
    use crate::trailer::bytes_to_digest;
    const P: u64 = 0xFFFF_FFFF_0000_0001;
    let mut canonical = [0u8; 32];
    canonical[..8].copy_from_slice(&5u64.to_le_bytes());
    let mut twin = canonical;
    twin[..8].copy_from_slice(&(5 + P).to_le_bytes());
    assert!(bytes_to_digest(&canonical).is_some());
    assert!(bytes_to_digest(&twin).is_none());
}

/// A word at or above p is refused wherever it appears in the trailer.
#[test]
fn a_sibling_word_at_or_above_p_is_refused() {
    let (root, t, ctx) = good();
    let mut b = t.clone();
    b[9..17].copy_from_slice(&0xFFFF_FFFF_0000_0001u64.to_le_bytes());
    assert!(refused(&root, &ctx, &b));
}

/// At a depth that leaves bits unused in the last direction byte, setting one
/// is refused.
#[test]
fn a_set_spare_direction_bit_is_refused() {
    let ctx = capsule_ctx(b"img", 0x7, EPOCH);
    let (root, tree) = enroll(&[(Kind::Capsule, ctx.clone())], 5);
    let t = tree.trailer(0).expect("trailer");
    assert!(verify(&root, 5, Kind::Capsule, &ctx, &t));
    let mut b = t.clone();
    let last = b.len() - 1;
    b[last] |= 0x80;
    assert!(!verify(&root, 5, Kind::Capsule, &ctx, &b));
}
