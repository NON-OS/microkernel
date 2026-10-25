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

//! The slices a surface is validated in cover its range exactly: no gap, no
//! overlap, each within one usercopy validation's reach.

use super::cap::{slices, SLICE};

const POLICY: &str = include_str!("../../../../src/usercopy/policy.rs");

fn cover(base: u64, len: u64, max: u64) -> Vec<(u64, u64)> {
    let got: Vec<_> = slices(base, len, max).collect();
    let mut at = base;
    for &(addr, n) in &got {
        assert_eq!(addr, at, "a gap or an overlap at {addr:#x}");
        assert!(n > 0 && n <= max, "slice of {n} bytes");
        at += n;
    }
    assert_eq!(at, base + len, "the range is not covered to its end");
    got
}

#[test]
fn a_range_under_one_slice_is_one_slice() {
    assert_eq!(cover(0x1000, 5000, 4096 * 4), vec![(0x1000, 5000)]);
}

#[test]
fn a_whole_number_of_slices_has_no_tail() {
    let got = cover(0x10_0000, 3 * 4096, 4096);
    assert_eq!(got.len(), 3);
    assert_eq!(got[2], (0x10_0000 + 2 * 4096, 4096));
}

#[test]
fn a_partial_last_slice_is_kept() {
    let got = cover(0x4000_0000, SLICE * 2 + 4096, SLICE);
    assert_eq!(got, vec![(0x4000_0000, SLICE), (0x4000_0000 + SLICE, SLICE), (0x4000_0000 + 2 * SLICE, 4096)]);
}

#[test]
fn every_length_to_the_ceiling_is_covered() {
    for len in [1, 4095, 4096, SLICE - 1, SLICE, SLICE + 1, 132_710_400, 128 * 1024 * 1024] {
        cover(0x7000_0000, len, SLICE);
    }
}

#[test]
fn the_slice_is_what_one_usercopy_validation_takes() {
    assert!(POLICY.contains("pub(super) const MAX_COPY_SIZE: usize = 64 * 1024 * 1024;"));
    assert_eq!(SLICE, 64 * 1024 * 1024);
}
