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

//! The SCSI this driver speaks (SPC-4, SBC-3), all of it mandatory for a UFS
//! logical unit (JESD220, 11.3), and the well-known LUN it asks for the list.

pub const TEST_UNIT_READY: u8 = 0x00;
pub const INQUIRY: u8 = 0x12;
pub const READ_CAPACITY_10: u8 = 0x25;
pub const READ_10: u8 = 0x28;
pub const WRITE_10: u8 = 0x2A;
pub const SYNCHRONIZE_CACHE_10: u8 = 0x35;
pub const READ_16: u8 = 0x88;
pub const WRITE_16: u8 = 0x8A;
/// SERVICE ACTION IN(16) with service action 0x10, READ CAPACITY(16).
pub const SERVICE_ACTION_IN_16: u8 = 0x9E;
pub const SA_READ_CAPACITY_16: u8 = 0x10;
pub const REPORT_LUNS: u8 = 0xA0;

pub const GOOD: u8 = 0x00;
pub const CHECK_CONDITION: u8 = 0x02;

pub const KEY_NOT_READY: u8 = 0x2;
pub const KEY_UNIT_ATTENTION: u8 = 0x6;

/// UFS_UPIU_REPORT_LUNS_WLUN: the well-known LU that answers REPORT LUNS.
pub const WLUN_REPORT_LUNS: u8 = 0x81;
/// Normal logical units are 0 to 127; bit 7 marks a well-known one.
pub const WLUN_BIT: u8 = 0x80;
