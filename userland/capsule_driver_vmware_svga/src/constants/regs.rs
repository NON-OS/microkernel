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

//! The SVGA II registers, from VMware's svga_reg.h. Each is reached by
//! writing its number to the index port and then using the value port.

pub const INDEX_PORT: u16 = 0;
pub const VALUE_PORT: u16 = 1;

pub const ID: u32 = 0;
pub const ENABLE: u32 = 1;
pub const WIDTH: u32 = 2;
pub const HEIGHT: u32 = 3;
pub const MAX_WIDTH: u32 = 4;
pub const MAX_HEIGHT: u32 = 5;
pub const BITS_PER_PIXEL: u32 = 7;
pub const BYTES_PER_LINE: u32 = 12;
pub const FB_OFFSET: u32 = 14;
pub const VRAM_SIZE: u32 = 15;
pub const CAPABILITIES: u32 = 17;
pub const MEM_SIZE: u32 = 19;
pub const CONFIG_DONE: u32 = 20;
pub const SYNC: u32 = 21;
pub const MEM_REGS: u32 = 30;

/// SVGA_MAKE_ID(2): the version this driver speaks.
pub const SVGA_ID_2: u32 = 0x9000_0002;
pub const CAP_EXTENDED_FIFO: u32 = 1 << 15;
/// SVGA_SYNC_GENERIC.
pub const SYNC_GENERIC: u32 = 1;
