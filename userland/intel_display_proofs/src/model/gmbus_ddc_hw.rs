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

//! GMBUS to a DDC/CI monitor at 0x37 in the model. GMBUS3 writes are kept
//! as they come; a write command to 0x37 takes its length's worth of them
//! (the first written before it), showing HW_RDY until all are in and then
//! the wait phase; the stop hands the message to the monitor. A read
//! command streams the monitor's reply (`gmbus_ddc_read_hw.rs`). Off its
//! pin pair, 0x37 is NAKed.

use super::gen9::Gen9;
use crate::engine::gmbus_regs::*;

pub struct Ci {
    pub write: bool,
    pub len: usize,
    pub nak: bool,
}

impl Gen9 {
    /// True when the write was the DDC/CI transaction's.
    pub(super) fn ci_write(&mut self, off: u32, v: u32) -> bool {
        let d = &mut self.ddc;
        if off == GMBUS3 {
            d.data.push(v);
            return d.ci.is_some();
        }
        if off != GMBUS1 || v & SW_RDY == 0 {
            return false;
        }
        if v & CYCLE_STOP != 0 && v & CYCLE_INDEX == 0 {
            let Some(ci) = d.ci.take() else { return false };
            let bytes: Vec<u8> = d.data.iter().flat_map(|w| w.to_le_bytes()).collect();
            if let (true, false, Some((_, m))) = (ci.write, ci.nak, d.mccs.as_mut()) {
                m.take(&bytes[..ci.len.min(bytes.len())]);
            }
            d.data.clear();
            d.active = false;
            return true;
        }
        if (v >> 1) & 0x7F != 0x37 {
            return false;
        }
        let pin = self.get(GMBUS0) & 7;
        let d = &mut self.ddc;
        let nak = d.mccs.as_ref().is_none_or(|m| m.0 != pin);
        // A write's first four bytes were put in GMBUS3 just before it.
        let first = d.data.last().copied().filter(|_| v & 1 == 0);
        d.data = first.into_iter().collect();
        d.ci = Some(Ci { write: v & 1 == 0, len: ((v >> 16) & 0x1FF) as usize, nak });
        (d.at, d.end) = (0, ((v >> 16) & 0x1FF) as usize);
        true
    }
}
