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

//! A box laid from a client's rows (wayland/damage_lay.rs) whose offsets do
//! not fit 32 bits, as a stride the client chose can make them: refused or
//! laid at its true offset, never wrapped onto another row.

use crate::damage_lay::lay;

/// A stride a client may send, near 2^32 / 4: one row of it past the first
/// is past u32 offsets.
const HUGE: u32 = u32::MAX / 4;

#[test]
fn a_box_on_the_last_row_of_a_huge_stride_is_refused_without_wrapping() {
    let client = vec![0xAAu8; 64];
    let mut out = vec![0u8; 64];
    let r = (3, 7, 4, 8);
    assert!(!lay(&mut out, (16, 0, 0), &client, HUGE, r, None));
    assert!(out.iter().all(|&b| b == 0), "nothing laid from a wrapped offset");
}

#[test]
fn a_window_row_of_a_huge_width_is_refused_without_wrapping() {
    let client = vec![0xAAu8; 64];
    let mut out = vec![0u8; 64];
    assert!(!lay(&mut out, (HUGE, u32::MAX, u32::MAX), &client, 16, (0, 0, 1, 1), Some(0)));
    assert!(out.iter().all(|&b| b == 0));
}

#[test]
fn a_box_that_fits_is_laid_where_it_lands() {
    let client: Vec<u8> = (0..32).collect();
    let mut out = vec![0u8; 48];
    assert!(lay(&mut out, (24, 1, 0), &client, 16, (1, 1, 3, 2), None));
    assert_eq!(&out[24 + 8..24 + 16], &client[16 + 4..16 + 12]);
    assert!(out[..32].iter().all(|&b| b == 0));
}
