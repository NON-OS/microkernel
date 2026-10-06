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
 * Known answers. The first is the digest the deployed PoseidonGoldilocks
 * commitNote is pinned to (stark_proofs::shield::test::pool_hash). The rest come
 * from a second, independent implementation written from the specification with
 * BLAKE3 taken from the b3sum binary, which reproduces that deployed digest first.
 * Do not re-baseline any of them: a change here is a different hash, and every
 * enrolled root would move with it.
 */

use crate::field::Fp;
use crate::leaf::{context_digest, leaf, leaf_of, pad_leaf, Kind};
use crate::measure::measure_digest_hybrid;
use crate::params::{RATE, WIDTH};
use crate::poseidon::Poseidon;

fn words<const N: usize>(v: [u64; N]) -> [Fp; N] {
    v.map(Fp::from_u64)
}

#[test]
fn the_deployed_commit_note_digest_holds() {
    let h = Poseidon::new();
    let mut p = [Fp::ZERO; 16];
    for (i, l) in p.iter_mut().take(11).enumerate() {
        *l = Fp::from_u64(i as u64 + 1);
    }
    p[11] = Fp::from_u64(0x4E4F_5445);
    let q = |i: usize| [p[i], p[i + 1], p[i + 2], p[i + 3]];
    let got = h.compress(&h.compress(&q(0), &q(4)), &h.compress(&q(8), &q(12)));
    let want =
        [6455909588408588117, 11340027322162162298, 9042362242223743603, 14573159163843564693];
    assert_eq!(got, words(want));
}

#[test]
fn the_permutation_of_one_to_eight_holds() {
    let got = Poseidon::new().permute(words([1, 2, 3, 4, 5, 6, 7, 8]));
    let want = [
        1022089083010806312,
        8134804760473441809,
        13972665140821454643,
        18290724068579387637,
        33538716422518085,
        4874145967630906442,
        3176566405087518195,
        7617985140508846139,
    ];
    assert_eq!(got, words(want));
}

/// The STARK tree's rate sponge: every round but the last.
#[test]
fn the_rate_sponge_of_one_to_four_holds() {
    let h = Poseidon::new();
    let mut s = [Fp::ZERO; WIDTH];
    s[..RATE].copy_from_slice(&words([1, 2, 3, 4]));
    for r in 0..31 {
        s = h.round(&s, r);
    }
    let want =
        [2169049970631926957, 6775483557278429577, 12065499352586199786, 5346927755136647619];
    assert_eq!(s[..RATE], words(want));
}

#[test]
fn the_first_round_constant_and_mds_entry_hold() {
    let h = Poseidon::new();
    assert_eq!(h.rc(0, 0).value(), 1416716216247504991);
    assert_eq!(h.mds(0, 0).value(), 2305843008676823040);
}

#[test]
fn the_hybrid_measurement_of_a_zero_digest_holds() {
    let got = measure_digest_hybrid(&Poseidon::new(), &[0u8; 32]);
    let want =
        [4376497421010222017, 14185657859492594963, 7190726597527500411, 15905283269163411124];
    assert_eq!(got, words(want));
}

#[test]
fn the_v3_leaf_of_a_zero_digest_holds_per_kind() {
    let h = Poseidon::new();
    let kernel =
        [243535016701144758, 17888283851409252096, 13809482232096691088, 3969615047475676197];
    let pad =
        [11717147524054051577, 11802451818334143455, 1531328350451364883, 8851161149773757722];
    assert_eq!(leaf_of(&h, Kind::Kernel, &[0u8; 32]), words(kernel));
    assert_eq!(leaf_of(&h, Kind::Pad, &[0u8; 32]), words(pad));
}

#[test]
fn the_v3_capsule_leaf_of_abc_holds() {
    let want =
        [4856894148767763545, 11745768569141934235, 14804755547324698195, 13895914039742190717];
    assert_eq!(leaf(&Poseidon::new(), Kind::Capsule, b"abc"), Some(words(want)));
    assert!(context_digest(b"abc").is_some());
}

#[test]
fn the_v3_pad_digest_holds() {
    let h = Poseidon::new();
    let mut d = [0u8; 32];
    let hex = b"796fdc03ef6cbd416957ae46c1c1e3cecf03032482e48330315ca4699b5d27f2";
    for (i, b) in d.iter_mut().enumerate() {
        let s = core::str::from_utf8(&hex[2 * i..2 * i + 2]).expect("hex");
        *b = u8::from_str_radix(s, 16).expect("hex");
    }
    assert_eq!(pad_leaf(&h, &[0u8; 32], 3), leaf_of(&h, Kind::Pad, &d));
}

/*
 * Kind 3 is the bootloader. Its leaf differs from every other kind's for the
 * same digest, so the anonymous proof can require an approved bootloader and an
 * approved kernel as two distinct openings.
 */
#[test]
fn the_v3_bootloader_leaf_holds() {
    let h = Poseidon::new();
    let zero =
        [18396667813487035579, 8303599323977009981, 18294702741523172963, 13929392303475005020];
    let abc = [5402826761677247289, 8634110419083413671, 12297564664387311797, 1804037806093469733];
    assert_eq!(leaf_of(&h, Kind::Bootloader, &[0u8; 32]), words(zero));
    assert_eq!(leaf(&h, Kind::Bootloader, b"abc"), Some(words(abc)));
    for other in [Kind::Kernel, Kind::Capsule, Kind::Pad] {
        assert_ne!(leaf_of(&h, other, &[0u8; 32]), words(zero));
    }
}
