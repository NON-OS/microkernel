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

//! The chip's IC version, from DUMMY_REG_RESET_0 (rts5227_get_ic_version);
//! an RTS522A of version A gets extra PHY values.

use crate::error::Result;
use crate::hw::read_register;
use crate::regs::pm::DUMMY_REG_RESET_0;
use crate::setup::Driver;

pub const IC_VER_A: u8 = 0;

pub fn read_ic_version(drv: &mut Driver) -> Result<()> {
    drv.ic_version = read_register(drv.regs, DUMMY_REG_RESET_0)? & 0x0F;
    Ok(())
}
