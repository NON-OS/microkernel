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

//! The EXT_CSD byte offsets the driver reads or writes (JESD84 eMMC 5.1,
//! as recalled, unverified), and the ones a new bus width must carry back
//! unchanged.

pub const EXT_CSD_LEN: usize = 512;

pub const FLUSH_CACHE: u8 = 32;
pub const CACHE_CTRL: u8 = 33;
pub const RPMB_SIZE_MULT: usize = 168;
pub const PARTITION_CONFIG: u8 = 179;
pub const BUS_WIDTH: u8 = 183;
pub const HS_TIMING: u8 = 185;
pub const EXT_CSD_REV: usize = 192;
pub const CSD_STRUCTURE: usize = 194;
pub const DEVICE_TYPE: usize = 196;
pub const PARTITION_SWITCH_TIME: usize = 199;
pub const SEC_COUNT: usize = 212;
pub const BOOT_SIZE_MULT: usize = 226;
pub const GENERIC_CMD6_TIME: usize = 248;
pub const CACHE_SIZE: usize = 249;

/// A bus width is kept only if EXT_CSD read at it gives back, byte for
/// byte, every field this driver goes on to use: the revision and
/// structure, the device type (speed), the size, boot and RPMB sizes, the
/// switch times and the cache size. All are fixed by the part, so a read
/// that differs was garbled on the new width.
pub const BUS_TEST_BYTES: [usize; 15] = [
    EXT_CSD_REV,
    CSD_STRUCTURE,
    DEVICE_TYPE,
    PARTITION_SWITCH_TIME,
    SEC_COUNT,
    SEC_COUNT + 1,
    SEC_COUNT + 2,
    SEC_COUNT + 3,
    BOOT_SIZE_MULT,
    RPMB_SIZE_MULT,
    GENERIC_CMD6_TIME,
    CACHE_SIZE,
    CACHE_SIZE + 1,
    CACHE_SIZE + 2,
    CACHE_SIZE + 3,
];
