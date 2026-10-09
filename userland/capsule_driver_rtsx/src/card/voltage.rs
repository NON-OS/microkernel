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

//! Signalling at 3.3 V, which the MMC core sets at every power-up
//! (rts5227_switch_output_voltage and rts522a_switch_output_voltage): the
//! PHY's output value for the family, then the pad drive for 3.3 V.

use crate::chip::Family;
use crate::error::Result;
use crate::hw::write_phy;
use crate::init::driving::fill_driving_3v3;
use crate::setup::Driver;
use crate::wire::CmdBuf;

pub fn signal_3v3(drv: &Driver) -> Result<()> {
    let value = match drv.family {
        Family::Rts5227 => 0x4FC0 | 0x24,
        Family::Rts522a => 0x57E4,
    };
    write_phy(drv.regs, 0x08, value)?;
    let mut buf = CmdBuf::new();
    fill_driving_3v3(&mut buf);
    crate::engine::send(drv, &buf, 100)
}
