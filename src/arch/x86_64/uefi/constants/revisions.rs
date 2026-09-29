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

/* The UEFI specification packs a revision as the major version in the upper
sixteen bits and, in the lower sixteen, the minor version times ten plus the
patch digit: 2.3.1 is (2 << 16) | 31 and 2.10 is (2 << 16) | 100. */
pub const fn uefi_revision(major: u16, minor: u16, patch: u16) -> u32 {
    ((major as u32) << 16) | (minor as u32 * 10 + patch as u32)
}

pub const UEFI_REVISION_2_0: u32 = uefi_revision(2, 0, 0);

pub const UEFI_REVISION_2_1: u32 = uefi_revision(2, 1, 0);

pub const UEFI_REVISION_2_3: u32 = uefi_revision(2, 3, 0);

pub const UEFI_REVISION_2_3_1: u32 = uefi_revision(2, 3, 1);

pub const UEFI_REVISION_2_4: u32 = uefi_revision(2, 4, 0);

pub const UEFI_REVISION_2_5: u32 = uefi_revision(2, 5, 0);

pub const UEFI_REVISION_2_6: u32 = uefi_revision(2, 6, 0);

pub const UEFI_REVISION_2_7: u32 = uefi_revision(2, 7, 0);

pub const UEFI_REVISION_2_8: u32 = uefi_revision(2, 8, 0);

pub const UEFI_REVISION_2_9: u32 = uefi_revision(2, 9, 0);

pub const UEFI_REVISION_2_10: u32 = uefi_revision(2, 10, 0);

pub const RESET_TYPE_COLD: u32 = 0;

pub const RESET_TYPE_WARM: u32 = 1;

pub const RESET_TYPE_SHUTDOWN: u32 = 2;

pub const RESET_TYPE_PLATFORM_SPECIFIC: u32 = 3;

pub const EFI_UNSPECIFIED_TIMEZONE: i16 = 0x07FF;

pub const EFI_TIME_ADJUST_DAYLIGHT: u8 = 0x01;

pub const EFI_TIME_IN_DAYLIGHT: u8 = 0x02;
