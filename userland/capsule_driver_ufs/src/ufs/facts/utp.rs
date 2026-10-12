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

//! The UTP transfer request descriptor, PRDT entry and command descriptor
//! layout. Source: JESD223, UTP transfer request chapter, as recalled,
//! unverified.

/// One UTRD, eight dwords.
pub const UTRD_LEN: usize = 32;
/// Command types in DW0 bits 31:28: 2.0 and later take UFS storage for
/// everything; 1.x take SCSI or device management.
pub const TYPE_SCSI: u32 = 0x0;
pub const TYPE_UFS_STORAGE: u32 = 0x1;
pub const TYPE_DEV_MANAGE: u32 = 0x2;
pub const TYPE_SHIFT: u32 = 28;
/// Data direction in DW0 bits 26:25: none, host to device, device to host.
pub const DIR_SHIFT: u32 = 25;
/// The OCS software writes before ringing; the host replaces it, 0 for
/// success. It sits in DW2 bits 7:0.
pub const OCS_INVALID: u32 = 0x0F;
pub const OCS_AT: usize = 8;

/// One PRDT entry: 64-bit address, a reserved dword, then the byte count
/// less one in an 18-bit field, dword granular.
pub const PRDT_ENTRY: usize = 16;
pub const PRDT_MAX_BYTES: usize = 1 << 18;

/// The command descriptor holds the request UPIU, the response UPIU and
/// the PRDT. Each part is placed on a 512-byte boundary here, which keeps
/// them 128-byte aligned as required and leaves room for the largest UPIU
/// this engine reads back.
pub const UPIU_AREA: usize = 512;
