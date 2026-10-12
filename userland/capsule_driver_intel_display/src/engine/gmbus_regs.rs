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

//! The south display's I2C controller, GMBUS (SKL PRM Vol 2c, GMBUS0 to
//! GMBUS4), which reads an HDMI or DVI monitor's EDID over DDC. Pin pair
//! numbers and bus cycle bits are as this driver reads the PRM, and are
//! unverified against the volume.

pub const GMBUS0: u32 = 0xC_5100;
pub const GMBUS1: u32 = 0xC_5104;
pub const GMBUS2: u32 = 0xC_5108;
pub const GMBUS3: u32 = 0xC_510C;

/// GMBUS0 pin pair select, 2:0, for the DDC lines of DDI B, C and D.
pub fn pin(ddi: u32) -> Option<u32> {
    match ddi {
        1 => Some(5),
        2 => Some(4),
        3 => Some(6),
        _ => None,
    }
}

/// GMBUS1: software clear interrupt, software ready, bus cycle (25 wait,
/// 26 index, 27 stop), byte count 24:16, index 15:8, address 7:1, read 0.
pub const SW_CLR_INT: u32 = 1 << 31;
pub const SW_RDY: u32 = 1 << 30;
pub const CYCLE_WAIT: u32 = 1 << 25;
pub const CYCLE_INDEX: u32 = 1 << 26;
pub const CYCLE_STOP: u32 = 1 << 27;

/// GMBUS2: hardware ready for the next four bytes, waiting phase after
/// the last, the slave's NAK, the bus active.
pub const HW_RDY: u32 = 1 << 11;
pub const HW_WAIT_PHASE: u32 = 1 << 14;
pub const NAK: u32 = 1 << 10;
pub const ACTIVE: u32 = 1 << 9;

/// The read command: `len` bytes from 7-bit `addr` starting at `index`.
pub fn read_cmd(addr: u32, index: u8, len: usize) -> u32 {
    SW_RDY
        | CYCLE_INDEX
        | CYCLE_WAIT
        | ((len as u32 & 0x1FF) << 16)
        | ((index as u32) << 8)
        | (addr << 1)
        | 1
}
