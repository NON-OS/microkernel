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

use super::shown::{shown_base, KIND_NONE};

const MMIO: u8 = 1;
const PIO: u8 = 2;

/// A boot's records as bars of (base, kind): an LPSS I2C host, a GPIO
/// controller (two community windows and its `_HID` bytes), a PCI device
/// with a port window, and the touchpad naming the host by device_id.
pub(super) const HOST_ID: u64 = 0x1_0000_0003;
pub(super) const HOST_BASE: u64 = 0xFE02_8000;
fn records() -> [[(u64, u8); 3]; 4] {
    [
        [(HOST_BASE, MMIO), (0, KIND_NONE), (0, KIND_NONE)],
        [(0xFD6D_0000, MMIO), (0xFD6E_0000, MMIO), (u64::from_le_bytes(*b"INT34C5\0"), KIND_NONE)],
        [(0xF000_0000, MMIO), (0xE000, PIO), (0, KIND_NONE)],
        [(HOST_ID, KIND_NONE), (0, KIND_NONE), (0, KIND_NONE)],
    ]
}

fn read_as(held: bool) -> Vec<u64> {
    records().iter().flatten().map(|&(base, kind)| shown_base(base, kind, held)).collect()
}

#[test]
fn a_holder_sees_its_bases() {
    let all: Vec<u64> = records().iter().flatten().map(|&(b, _)| b).collect();
    assert_eq!(read_as(true), all);
}

#[test]
fn a_non_holder_sees_zeros_for_every_window() {
    for (got, &(base, kind)) in read_as(false).iter().zip(records().iter().flatten()) {
        let want = if kind == KIND_NONE { base } else { 0 };
        assert_eq!(*got, want, "kind {kind}");
    }
    assert_eq!(shown_base(0xF000_0000, 3, false), 0, "an unknown kind is not shown");
}

#[test]
fn no_physical_address_in_what_a_non_holder_reads() {
    let windows: Vec<u64> =
        records().iter().flatten().filter(|&&(_, k)| k != KIND_NONE).map(|&(b, _)| b).collect();
    let read = read_as(false);
    assert!(windows.iter().all(|w| !read.contains(w)), "{read:x?}");
    assert!(!read.contains(&HOST_BASE), "the touchpad names its host, not its address");
}
