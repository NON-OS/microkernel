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

//! An access names a register when its device, function, width and
//! alignment are legal and it lies in the 4 KiB a PCIe function decodes.

use super::config::check::check;
use super::error::PciError;
use super::types::PciAddress;

const A: PciAddress = PciAddress::new(0, 0, 0);

#[test]
fn every_aligned_access_in_4k_is_a_register() {
    for width in [1u8, 2, 4] {
        let mut offset = 0u16;
        while offset < 0x1000 {
            assert_eq!(check(A, offset, width), Ok(()), "{offset:#x}/{width}");
            offset += width as u16;
        }
    }
    assert_eq!(check(PciAddress::in_segment(0x1000, 255, 31, 7), 0xFFC, 4), Ok(()));
}

#[test]
fn past_4k_is_not_a_register() {
    assert_eq!(check(A, 0x1000, 1), Err(PciError::InvalidOffset(0x1000)));
    assert_eq!(check(A, 0xFFFF, 1), Err(PciError::InvalidOffset(0xFFFF)));
    assert_eq!(check(A, 0x1000, 4), Err(PciError::InvalidOffset(0x1000)));
}

#[test]
fn misaligned_and_odd_widths_are_refused() {
    let unaligned = |offset, alignment| Err(PciError::UnalignedAccess { offset, alignment });
    assert_eq!(check(A, 1, 2), unaligned(1, 2));
    assert_eq!(check(A, 0x102, 4), unaligned(0x102, 4));
    assert_eq!(check(A, 0xFFE, 4), unaligned(0xFFE, 4));
    for width in [0u8, 3, 8] {
        assert_eq!(check(A, 0, width), unaligned(0, width));
    }
}

#[test]
fn device_and_function_out_of_range_are_refused() {
    assert_eq!(check(PciAddress::new(0, 32, 0), 0, 4), Err(PciError::InvalidDevice(32)));
    assert_eq!(check(PciAddress::new(0, 0, 8), 0, 4), Err(PciError::InvalidFunction(8)));
}
