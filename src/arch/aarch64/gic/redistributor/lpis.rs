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

use super::constants::GICR_CTLR;
use super::device::GicRedistributor;

const GICR_PROPBASER: u64 = 0x0070;
const GICR_PENDBASER: u64 = 0x0078;
const CTLR_ENABLE_LPIS: u32 = 1 << 0;
const TYPER_PLPIS: u64 = 1 << 0;

impl GicRedistributor {
    /// `GICR_TYPER.PLPIS`: this frame takes physical LPIs.
    pub fn takes_lpis(&self) -> bool {
        self.typer() & TYPER_PLPIS != 0
    }

    /// What an ITS without `PTA` names this frame by.
    pub fn processor_number(&self) -> u16 {
        (self.typer() >> 8) as u16
    }

    /// Turn LPIs off so the tables can be replaced. Firmware or an earlier
    /// kernel may have left them on with tables of its own, which the GIC
    /// would go on reading. Clearing the bit need not be implemented, so
    /// the bit is read back: still set means the tables cannot be ours.
    pub fn stop_lpis(&self) -> Result<(), &'static str> {
        let ctlr = self.read_reg(GICR_CTLR);
        if ctlr & CTLR_ENABLE_LPIS == 0 {
            return Ok(());
        }
        self.write_reg(GICR_CTLR, ctlr & !CTLR_ENABLE_LPIS);
        self.settle()?;
        if self.read_reg(GICR_CTLR) & CTLR_ENABLE_LPIS != 0 {
            return Err("LPIs were left on and this redistributor will not turn them off");
        }
        Ok(())
    }

    /// Write both table registers and return what they read back, whose
    /// shareability says whether the GIC snoops the CPU's caches.
    pub fn set_lpi_tables(&self, propbaser: u64, pendbaser: u64) -> (u64, u64) {
        self.write_reg64(GICR_PROPBASER, propbaser);
        self.write_reg64(GICR_PENDBASER, pendbaser);
        (self.read_reg64(GICR_PROPBASER), self.read_reg64(GICR_PENDBASER))
    }

    pub fn start_lpis(&self) {
        let ctlr = self.read_reg(GICR_CTLR);
        self.write_reg(GICR_CTLR, ctlr | CTLR_ENABLE_LPIS);
        // The tables are in use from here; nothing after may race ahead.
        // SAFETY: a barrier; it touches no memory and changes no state.
        unsafe { core::arch::asm!("dsb sy", options(nostack, preserves_flags)) }
    }
}
