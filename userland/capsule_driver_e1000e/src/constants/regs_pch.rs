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

//! BAR0 registers only the PCH parts (I217, I218, I219) have, from Linux
//! e1000e regs.h and ich8lan.h.

pub const REG_FEXTNVM3: usize = 0x003C;
pub const REG_FEXTNVM7: usize = 0x00E4;
pub const REG_IOSFPC: usize = 0x0F28;
pub const REG_KABGTXD: usize = 0x3004;
pub const REG_H2ME: usize = 0x5B50;
pub const REG_FWSM: usize = 0x5B54;
pub const REG_FEXTNVM9: usize = 0x5BB4;
pub const REG_FEXTNVM11: usize = 0x5BBC;
pub const REG_FEXTNVM12: usize = 0x5BC0;
pub const REG_FFLT_DBG: usize = 0x5F04;
