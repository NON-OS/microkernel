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

//! A DMA grant is used only as asked: long enough, page aligned on both
//! sides, never at address zero, never wrapping.

use crate::setup::grant::usable;

const VA: u64 = 0x7000_0000_0000;
const DA: u64 = 0x10_0000;

#[test]
fn a_grant_that_covers_the_request_is_used() {
    assert!(usable(VA, DA, 0x3000, 0x3000));
    assert!(usable(VA, DA, 0x4000, 0x3000));
    assert!(usable(VA, !0xFFF_u64 - 0x3000, 0x3000, 0x3000), "ends at the top");
}

#[test]
fn a_short_misplaced_or_wrapping_grant_is_refused() {
    let want = 0x3000;
    for (va, da, len) in [
        (VA, DA, 0x2FFF),
        (VA, DA, 0),
        (0, DA, want as u64),
        (VA, 0, want as u64),
        (VA + 8, DA, want as u64),
        (VA, DA + 0x800, want as u64),
        (!0xFFF_u64, DA, want as u64),
        (VA, !0xFFF_u64, want as u64),
    ] {
        assert!(!usable(va, da, len, want), "va {va:#x} da {da:#x} len {len:#x}");
    }
    assert!(!usable(VA, DA, u64::MAX, usize::MAX));
}
