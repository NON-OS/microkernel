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

//! The EXT_CSD byte offsets the driver reads or writes, and the read-only
//! ones the bus width test compares.

pub const EXT_CSD_LEN: usize = 512;

pub const FLUSH_CACHE: u8 = 32;
pub const CACHE_CTRL: u8 = 33;
pub const PARTITION_SUPPORT: usize = 160;
pub const RPMB_SIZE_MULT: usize = 168;
pub const PARTITION_CONFIG: u8 = 179;
pub const ERASED_MEM_CONT: usize = 181;
pub const BUS_WIDTH: u8 = 183;
pub const HS_TIMING: u8 = 185;
pub const EXT_CSD_REV: usize = 192;
pub const CSD_STRUCTURE: usize = 194;
pub const DEVICE_TYPE: usize = 196;
pub const PARTITION_SWITCH_TIME: usize = 199;
pub const SEC_COUNT: usize = 212;
pub const S_A_TIMEOUT: usize = 217;
pub const HC_WP_GRP_SIZE: usize = 221;
pub const ERASE_TIMEOUT_MULT: usize = 223;
pub const HC_ERASE_GRP_SIZE: usize = 224;
pub const BOOT_SIZE_MULT: usize = 226;
pub const SEC_TRIM_MULT: usize = 229;
pub const SEC_ERASE_MULT: usize = 230;
pub const SEC_FEATURE_SUPPORT: usize = 231;
pub const TRIM_MULT: usize = 232;
pub const GENERIC_CMD6_TIME: usize = 248;
pub const CACHE_SIZE: usize = 249;

/// The bytes Linux's mmc_compare_ext_csds holds equal across a bus width
/// change: read-only fields a corrupted transfer would not reproduce.
pub const BUS_TEST_BYTES: [usize; 17] = [
    PARTITION_SUPPORT,
    ERASED_MEM_CONT,
    EXT_CSD_REV,
    CSD_STRUCTURE,
    DEVICE_TYPE,
    S_A_TIMEOUT,
    HC_WP_GRP_SIZE,
    ERASE_TIMEOUT_MULT,
    HC_ERASE_GRP_SIZE,
    SEC_TRIM_MULT,
    SEC_ERASE_MULT,
    SEC_FEATURE_SUPPORT,
    TRIM_MULT,
    SEC_COUNT,
    SEC_COUNT + 1,
    SEC_COUNT + 2,
    SEC_COUNT + 3,
];
