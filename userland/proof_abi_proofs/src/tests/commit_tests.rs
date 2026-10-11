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

//! The two chains held to the bare permutation: lane 0 the domain, then each
//! block of four absorbed as the right half of one permutation's input.

use nonos_proof_abi::{allows, bucket, hist_commit, side, syscall_root, HIST_WORDS};
use nonos_stark::air::{Poseidon, RATE};
use nonos_stark::field::Fp;

fn by_permutation(domain: u64, words: &[u64]) -> [u64; 4] {
    let h = Poseidon::new(5, [Fp::ZERO; RATE]);
    let mut acc = [Fp::from_u64(domain), Fp::ZERO, Fp::ZERO, Fp::ZERO];
    for b in words.chunks(4) {
        let mut s = [Fp::ZERO; 8];
        s[..4].copy_from_slice(&acc);
        (0..4).for_each(|i| s[4 + i] = Fp::from_u64(b[i]));
        acc.copy_from_slice(&h.permute(s)[..4]);
    }
    acc.map(Fp::to_u64)
}

#[test]
fn hc_is_the_histogram_then_two_zeros_then_the_salt() {
    let mut h = [0u32; HIST_WORDS];
    h[0] = 5;
    h[1025] = u32::MAX;
    let salt = [1, 2, 3, 4];
    let mut w: Vec<u64> = h.iter().map(|&c| u64::from(c)).collect();
    w.extend([0, 0, 1, 2, 3, 4]);
    assert_eq!(hist_commit(&h, &salt), Some(by_permutation(0x4E4F_4E4F_5349_534F, &w)));
    assert_ne!(hist_commit(&h, &[1, 2, 3, 5]), hist_commit(&h, &salt), "the salt hides H");
    assert_eq!(hist_commit(&h, &[0xFFFF_FFFF_0000_0001, 0, 0, 0]), None);
}

#[test]
fn a_is_513_bits_then_three_zeros() {
    let mut set = [0u8; 65];
    set[0] = 0b11;
    set[64] = 1;
    let mut w = vec![0u64; 516];
    (w[0], w[1], w[512]) = (1, 1, 1);
    assert_eq!(syscall_root(&set), Some(by_permutation(0x4E4F_4E4F_5350_4F4C, &w)));
    assert!(allows(&set, 1) && allows(&set, 512) && !allows(&set, 2) && !allows(&set, 600));
}

#[test]
fn a_bit_past_512_has_no_root() {
    let mut set = [0u8; 65];
    set[64] = 0b10;
    assert_eq!(syscall_root(&set), None);
}

#[test]
fn buckets_and_sides_at_their_edges() {
    assert_eq!((bucket(0), bucket(511), bucket(512), bucket(u64::MAX)), (0, 511, 512, 512));
    let (eperm, enosys, ebadf) = (-1i64 as u64, -38i64 as u64, -9i64 as u64);
    assert_eq!((side(0), side(i64::MAX as u64), side(1 << 63), side(ebadf)), (0, 0, 0, 0));
    assert_eq!((side(eperm), side(enosys)), (1, 1), "only the two refusals");
}
