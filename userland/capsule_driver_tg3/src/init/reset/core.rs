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

//! The reset itself. The core-clock reset clears Memory Space in PCI
//! Command, so the saved bits go back through configuration space before
//! the registers are touched again.

use crate::constants::grc::{
    GRC_FASTBOOT_PC, GRC_MISC_CFG, GRC_MISC_CFG_CORECLK_RESET, GRC_MISC_CFG_PCIE_BIT29,
};
use crate::constants::timeouts::CORE_RESET_HOLD_MS;
use crate::init::wait;
use crate::setup::Driver;

use super::{after, nvram, pcie};

const PCI_COMMAND: u32 = 0x04;

pub fn chip_reset(d: &mut Driver) -> Result<(), &'static str> {
    let _ = nvram::lock(&d.regs);
    let command = d.cfg.read(PCI_COMMAND, 2)? as u16;
    // SAFETY: `d.regs` carries the broker MmioMap base for BAR0 and both
    // GRC registers are inside it.
    unsafe {
        d.regs.w32(GRC_FASTBOOT_PC, 0);
        // PCI Express, not a 5750 A0: bit 29 first, then with the reset.
        d.regs.w32(GRC_MISC_CFG, GRC_MISC_CFG_PCIE_BIT29);
        d.regs.w32(GRC_MISC_CFG, GRC_MISC_CFG_CORECLK_RESET | GRC_MISC_CFG_PCIE_BIT29);
    }
    // Registers do not answer here; a configuration read pushes the write
    // out, between the two holds Linux takes.
    wait::hold(CORE_RESET_HOLD_MS);
    let _ = d.cfg.read(PCI_COMMAND, 4)?;
    wait::hold(CORE_RESET_HOLD_MS);
    pcie::clear_after_reset(&d.cfg, d.pcie_cap)?;
    pcie::restore(&d.cfg, command)?;
    after::run(d)
}
