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

//! The display's dynamic DC states (SKL PRM Vol 2c, DC_STATE_EN; Vol 12,
//! display power management). In DC5 and DC6 the hardware powers down
//! display state that only the DMC firmware saves and restores, so with
//! no DMC loaded by this driver they are switched off before anything
//! else is touched: firmware may have left them on with a DMC this driver
//! did not seal. Gen8 has no DC states and never calls this.

use super::access::{rmw, Expiry, Mmio};
use super::error::Error;
use super::wait::wait_for;

pub const DC_STATE_EN: u32 = 0x4_5504;
/// Dynamic DC State Enable, 1:0: 01b DC5, 10b DC6, 00b neither (field
/// position unverified against the volume).
pub const DC_MASK: u32 = 0x3;

/// Switch DC5 and DC6 off and wait until the register reads back off.
pub fn dc_off(m: &mut dyn Mmio, until: &dyn Expiry) -> Result<(), Error> {
    if m.read(DC_STATE_EN) & DC_MASK == 0 {
        return Ok(());
    }
    rmw(m, DC_STATE_EN, DC_MASK, 0);
    wait_for(m, DC_STATE_EN, DC_MASK, 0, until)
}
