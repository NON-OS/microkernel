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

//! A UTP Transfer Request Descriptor (JESD223, 6.1.1), as ufshcd_prepare_req
//! _desc_hdr and ufshcd_host_memory_configure fill it: command type and data
//! direction in DW0, an OCS the host overwrites in DW2, the command
//! descriptor's address, and the response and PRDT offsets in dwords.

use super::layout::{PRDT_AT, RSP_AT, UPIU_AREA, UTRD_LEN};

/// UTP_CMD_TYPE_UFS_STORAGE, what UFSHCI 2.0 and later take for every
/// request; 1.x takes UTP_CMD_TYPE_SCSI or UTP_CMD_TYPE_DEV_MANAGE.
pub const TYPE_UFS_STORAGE: u32 = 0x1;
pub const TYPE_SCSI: u32 = 0x0;
pub const TYPE_DEV_MANAGE: u32 = 0x2;
/// OCS_INVALID_COMMAND_STATUS: written by software, replaced by the host.
pub const OCS_INVALID: u32 = 0x0F;

/// Data direction in DW0 bits 26:25.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    None = 0,
    ToDevice = 1 << 25,
    FromDevice = 2 << 25,
}

/// The descriptor for a request whose command descriptor is at `ucd_bus`,
/// with `prdt` PRDT entries. No interrupt is asked for: the driver polls.
pub fn utrd(cmd_type: u32, dir: Dir, ucd_bus: u64, prdt: u16) -> [u8; UTRD_LEN] {
    let dw: [u32; 8] = [
        cmd_type << 28 | dir as u32,
        0,
        OCS_INVALID,
        0,
        ucd_bus as u32,
        (ucd_bus >> 32) as u32,
        ((RSP_AT / 4) as u32) << 16 | (UPIU_AREA / 4) as u32,
        ((PRDT_AT / 4) as u32) << 16 | prdt as u32,
    ];
    let mut out = [0u8; UTRD_LEN];
    for (i, w) in dw.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
    }
    out
}

/// The OCS the host left in DW2 bits 7:0; 0 is success.
pub fn ocs(desc: &[u8; UTRD_LEN]) -> u8 {
    desc[8]
}
