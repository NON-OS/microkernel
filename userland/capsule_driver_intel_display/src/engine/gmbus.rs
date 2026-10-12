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

//! An EDID read over DDC through GMBUS: select the DDI's pins, ask for the
//! bytes from address 0x50 at an index, take them four at a time as the
//! controller has them ready, then end with a stop. A NAK clears the
//! controller and is reported; every wait has a deadline.

use super::access::{Clock, Expiry, Mmio, Until};
use super::error::Error;
use super::gmbus_regs::*;
use super::gmbus_take::{take, STEP_US};

/// Read `out.len()` bytes (at most 256) of the EDID on DDI `ddi`'s DDC
/// pins, from byte `index`.
pub fn read_edid(
    m: &mut dyn Mmio,
    clock: &dyn Clock,
    ddi: u32,
    index: u8,
    out: &mut [u8],
) -> Result<(), Error> {
    let p = pin(ddi).ok_or(Error::Argument)?;
    if out.is_empty() || out.len() > 256 {
        return Err(Error::Argument);
    }
    m.write(GMBUS0, p);
    m.write(GMBUS1, read_cmd(0x50, index, out.len()));
    let r = take(m, clock, out);
    if r.is_err() {
        m.write(GMBUS1, SW_CLR_INT);
        m.write(GMBUS1, 0);
    }
    m.write(GMBUS1, SW_RDY | CYCLE_STOP);
    let until = Until::new(clock, STEP_US);
    while m.read(GMBUS2) & ACTIVE != 0 && !until.expired() {}
    m.write(GMBUS0, 0);
    r
}
