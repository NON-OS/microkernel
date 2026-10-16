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

//! The host registers in the BAR. A reader that left the bus reads all
//! ones and takes nothing; one whose link blinks reads all ones on BIPR
//! for a few polls and takes every write, as a link that came back does.

use nonos_libc::note;

use super::reader::Reader;
use crate::regs::host::*;

impl Reader {
    pub fn r32(&mut self, offset: u32) -> u32 {
        if self.knobs.gone {
            return u32::MAX;
        }
        match offset {
            BIPR if self.knobs.blink > 0 => {
                self.knobs.blink -= 1;
                u32::MAX
            }
            BIPR => self.bipr(),
            HAIMR => self.haimr,
            HCBAR => self.hcbar,
            HDBAR => self.hdbar,
            BIER => self.bier,
            _ => 0,
        }
    }

    pub fn w32(&mut self, offset: u32, value: u32) {
        if self.knobs.gone {
            return;
        }
        match offset {
            HCBAR => self.hcbar = value,
            HCBCTLR if value & STOP_CMD != 0 => self.stop_cmd(),
            HCBCTLR if value & START_CMD != 0 => self.start_cmd(value),
            HDBAR => self.hdbar = value,
            HDBCTLR if value & STOP_DMA != 0 => self.stop_dma(),
            HDBCTLR if value & TRIG_DMA != 0 => self.adma(value),
            HAIMR => self.haimr = self.access(value),
            BIPR => self.pending &= !value,
            BIER => {
                note(format!("bier {value:#x}"));
                self.bier = value;
            }
            _ => {}
        }
    }
}
