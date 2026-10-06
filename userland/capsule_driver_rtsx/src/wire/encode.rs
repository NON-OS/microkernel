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

//! The words the host registers and the command buffer take, as
//! rtsx_pci_add_cmd, rtsx_pci_write_register, rtsx_pci_send_cmd,
//! rtsx_pci_add_sg_tbl and rtsx_pci_dma_transfer build them.

use super::kind::CmdKind;
use crate::regs::host::{
    ADMA_MODE, DEVICE_TO_HOST, HAIMR_TRANS_START, HAIMR_WRITE, HW_AUTO_RSP, START_CMD, TRIG_DMA,
};

const RTSX_SG_VALID: u64 = 0x01;
const RTSX_SG_END: u64 = 0x02;
const RTSX_SG_TRANS_DATA: u64 = 0x02 << 4;

/// One command buffer entry: kind in bits 31:30, register in 29:16, mask
/// in 15:8, data in 7:0.
pub const fn cmd_entry(kind: CmdKind, reg: u16, mask: u8, data: u8) -> u32 {
    ((kind as u32 & 0x03) << 30)
        | ((reg as u32 & 0x3FFF) << 16)
        | ((mask as u32) << 8)
        | data as u32
}

pub const fn haimr_read(reg: u16) -> u32 {
    HAIMR_TRANS_START | ((reg as u32 & 0x3FFF) << 16)
}

pub const fn haimr_write(reg: u16, mask: u8, data: u8) -> u32 {
    HAIMR_TRANS_START
        | HAIMR_WRITE
        | ((reg as u32 & 0x3FFF) << 16)
        | ((mask as u32) << 8)
        | data as u32
}

/// The chip clears TRANS_START when the access is done; the low byte then
/// holds the register's value.
pub const fn haimr_done(value: u32) -> bool {
    value & HAIMR_TRANS_START == 0
}

/// Start `count` entries, with the read results written back over the
/// buffer (the hardware auto response).
pub const fn hcbctlr(count: usize) -> u32 {
    START_CMD | HW_AUTO_RSP | ((count as u32 * 4) & 0x00FF_FFFF)
}

/// Start ADMA from the card to memory.
pub const fn hdbctlr_read() -> u32 {
    ((DEVICE_TO_HOST & 0x01) << 29) | TRIG_DMA | ADMA_MODE
}

/// One scatter-gather entry for every chip but the RTS5261 and RTS5228:
/// a 32-bit address in the high half, the length from bit 12.
pub const fn sg_entry(addr: u32, len: u32, end: bool) -> u64 {
    let option = RTSX_SG_VALID | RTSX_SG_TRANS_DATA | if end { RTSX_SG_END } else { 0 };
    ((addr as u64) << 32) | ((len as u64) << 12) | option
}
