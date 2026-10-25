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

//! What the driver reads of GMBUS while a DDC/CI transaction is up: NAK
//! off the monitor's pins, HW_RDY until the length's bytes have gone, then
//! the wait phase; and in a read, the monitor's reply four bytes at a time.

use super::gen9::Gen9;
use crate::engine::gmbus_regs::*;

impl Gen9 {
    pub(super) fn ci_read(&mut self, off: u32) -> Option<u32> {
        let d = &mut self.ddc;
        let ci = d.ci.as_ref()?;
        let have = if ci.write { d.data.len() * 4 } else { d.at };
        match off {
            GMBUS2 if ci.nak => Some(NAK | ACTIVE),
            GMBUS2 if have < ci.len => Some(HW_RDY | ACTIVE),
            GMBUS2 => Some(HW_WAIT_PHASE | ACTIVE),
            GMBUS3 if !ci.write => {
                let m = &mut d.mccs.as_mut()?.1;
                let w = [m.next_byte(), m.next_byte(), m.next_byte(), m.next_byte()];
                d.at += 4;
                Some(u32::from_le_bytes(w))
            }
            _ => None,
        }
    }
}
