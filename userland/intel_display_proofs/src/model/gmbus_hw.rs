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

//! GMBUS in the model: a monitor's EDID on one pin pair. A read command
//! on that pair streams the EDID from its index four bytes per HW_RDY,
//! then shows the wait phase; on any other pair the address is NAKed.

use super::gen9::Gen9;
use super::gmbus_state::Ddc;
use crate::engine::gmbus_regs::*;

impl Gen9 {
    pub(super) fn gmbus_write(&mut self, off: u32, v: u32) {
        if self.ci_write(off, v) {
            return;
        }
        if off != GMBUS1 || v & SW_RDY == 0 {
            return;
        }
        if v & CYCLE_STOP != 0 && v & CYCLE_INDEX == 0 {
            self.ddc.active = false;
            return;
        }
        let pin = self.get(GMBUS0) & 7;
        let hit = self.ddc.monitor.as_ref().is_some_and(|m| m.0 == pin) && (v >> 1) & 0x7F == 0x50;
        let index = ((v >> 8) & 0xFF) as usize;
        self.ddc = Ddc {
            monitor: self.ddc.monitor.take(),
            at: index,
            end: index + ((v >> 16) & 0x1FF) as usize,
            nak: !hit,
            active: true,
            mccs: self.ddc.mccs.take(),
            ci: None,
            data: Vec::new(),
        };
    }

    pub(super) fn gmbus_read(&mut self, off: u32) -> Option<u32> {
        if self.ddc.ci.is_some() {
            return self.ci_read(off);
        }
        let d = &mut self.ddc;
        match off {
            GMBUS2 if d.nak => Some(NAK | ACTIVE),
            GMBUS2 if d.at < d.end => Some(HW_RDY | ACTIVE),
            GMBUS2 => Some(if d.active { HW_WAIT_PHASE | ACTIVE } else { 0 }),
            GMBUS3 => {
                let edid = &d.monitor.as_ref()?.1;
                let mut w = [0u8; 4];
                for b in w.iter_mut() {
                    *b = *edid.get(d.at).unwrap_or(&0);
                    d.at += 1;
                }
                Some(u32::from_le_bytes(w))
            }
            _ => None,
        }
    }
}
