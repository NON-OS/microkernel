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

use crate::arch::x86_64::uefi::constants::*;
use crate::arch::x86_64::uefi::variable::FirmwareInfo;

#[test]
fn revisions_are_the_specification_values() {
    assert_eq!(UEFI_REVISION_2_0, 0x0002_0000);
    assert_eq!(UEFI_REVISION_2_1, 0x0002_000A);
    assert_eq!(UEFI_REVISION_2_3, 0x0002_001E);
    assert_eq!(UEFI_REVISION_2_3_1, 0x0002_001F);
    assert_eq!(UEFI_REVISION_2_4, 0x0002_0028);
    assert_eq!(UEFI_REVISION_2_5, 0x0002_0032);
    assert_eq!(UEFI_REVISION_2_6, 0x0002_003C);
    assert_eq!(UEFI_REVISION_2_7, 0x0002_0046);
    assert_eq!(UEFI_REVISION_2_8, 0x0002_0050);
    assert_eq!(UEFI_REVISION_2_9, 0x0002_005A);
    assert_eq!(UEFI_REVISION_2_10, 0x0002_0064);
}

#[test]
fn revisions_order_as_versions() {
    let ordered = [
        UEFI_REVISION_2_0,
        UEFI_REVISION_2_1,
        UEFI_REVISION_2_3,
        UEFI_REVISION_2_3_1,
        UEFI_REVISION_2_4,
        UEFI_REVISION_2_5,
        UEFI_REVISION_2_6,
        UEFI_REVISION_2_7,
        UEFI_REVISION_2_8,
        UEFI_REVISION_2_9,
        UEFI_REVISION_2_10,
    ];
    for pair in ordered.windows(2) {
        assert!(pair[0] < pair[1], "{:#x} {:#x}", pair[0], pair[1]);
    }
}

#[test]
fn the_firmware_record_reads_2_8_back() {
    let info = FirmwareInfo { revision: UEFI_REVISION_2_8, ..FirmwareInfo::default() };
    assert_eq!(info.uefi_major_version(), 2);
    assert_eq!(info.uefi_minor_version(), 80);
}
