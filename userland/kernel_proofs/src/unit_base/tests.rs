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

use super::base::{check_base, BadBase, AMD_WINDOW, DRHD_WINDOW};

/// Low memory, then RAM from 4 GiB up, as a 16 GiB machine's map gives it.
const RAM: [(u64, u64); 2] = [(0x10_0000, 0x7F00_0000), (0x1_0000_0000, 0x4_8000_0000)];
const BITS: u32 = 39;
const VTD: u64 = 0xFED9_0000;

fn check(base: u64, window: u64, kept: &[u64]) -> Result<(), BadBase> {
    check_base(base, window, BITS, &RAM, kept)
}

#[test]
fn a_real_unit_base_is_admitted() {
    assert_eq!(check(VTD, DRHD_WINDOW, &[]), Ok(()));
    assert_eq!(check(0xFED9_1000, DRHD_WINDOW, &[VTD]), Ok(()), "the next page");
    assert_eq!(check(0xFD20_0000, AMD_WINDOW, &[]), Ok(()));
}

#[test]
fn a_base_that_is_not_a_register_window_is_refused() {
    assert_eq!(check(0, DRHD_WINDOW, &[]), Err(BadBase::Zero));
    assert_eq!(check(VTD + 0x800, DRHD_WINDOW, &[]), Err(BadBase::Unaligned));
    assert_eq!(check(1 << BITS, DRHD_WINDOW, &[]), Err(BadBase::PastAddressWidth));
    let last = (1 << BITS) - DRHD_WINDOW;
    assert_eq!(check(last, DRHD_WINDOW, &[]), Ok(()), "ends at the width");
    assert_eq!(check(!0xFFF_u64, DRHD_WINDOW, &[]), Err(BadBase::PastAddressWidth));
}

#[test]
fn a_base_over_ram_or_another_unit_is_refused() {
    assert_eq!(check(0x20_0000, DRHD_WINDOW, &[]), Err(BadBase::OverRam));
    assert_eq!(check(0x7EFF_F000, DRHD_WINDOW, &[]), Err(BadBase::OverRam), "last RAM page");
    assert_eq!(check(0x7F00_0000, DRHD_WINDOW, &[]), Ok(()), "just past it");
    assert_eq!(check(VTD, DRHD_WINDOW, &[VTD]), Err(BadBase::OverAnotherUnit));
    let inside = 0xFD20_2000;
    assert_eq!(check(inside, AMD_WINDOW, &[0xFD20_0000]), Err(BadBase::OverAnotherUnit));
}
