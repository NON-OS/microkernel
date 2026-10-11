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

//! What the claimed function is, read as tg3_get_invariants and
//! tg3_detect_asic_rev read it, and the host-control word written before
//! the first register access, since the swaps and indirect access it sets
//! are what every later access relies on.

use crate::constants::chip::{asic_rev, is_57765_class, ASIC_REV_USE_PROD_ID_REG};
use crate::constants::grc::{MEMARB_MODE, MEMARB_MODE_ENABLE};
use crate::constants::pcicfg::{
    MISC_HOST_CTRL_CHIPREV_SHIFT, TG3PCI_GEN15_PRODID_ASICREV, TG3PCI_MISC_HOST_CTRL,
};
use crate::regs::Regs;
use crate::setup::config::Config;

use super::hw_cfg::HwCfg;
use super::pcicfg::host_control;

#[derive(Clone, Copy, Debug)]
pub struct Chip {
    /// tp->pci_chip_rev_id.
    pub rev: u32,
    pub hw: HwCfg,
}

/// The chip and its PCI Express capability's offset.
pub fn identify(cfg: &Config, regs: &Regs) -> Result<(Chip, u32), &'static str> {
    let misc = cfg.read(TG3PCI_MISC_HOST_CTRL, 4)?;
    let mut rev = misc >> MISC_HOST_CTRL_CHIPREV_SHIFT;
    if asic_rev(rev) == ASIC_REV_USE_PROD_ID_REG {
        rev = cfg.read(TG3PCI_GEN15_PRODID_ASICREV, 4)?;
    }
    if !is_57765_class(rev) {
        return Err("chip revision is not of the 57765 class");
    }
    let low = cfg.read(TG3PCI_MISC_HOST_CTRL, 2)? as u16;
    cfg.write16(TG3PCI_MISC_HOST_CTRL, host_control(low))
        .map_err(|_| "host control write refused")?;
    let pcie_cap = cfg.pcie_capability()?;
    // SAFETY: `regs` carries the broker MmioMap base for BAR0, MEMARB_MODE
    // inside it. The arbiter has to run for SRAM reads to answer.
    unsafe { regs.w32(MEMARB_MODE, regs.r32(MEMARB_MODE) | MEMARB_MODE_ENABLE) };
    let hw = HwCfg::read(regs)?;
    Ok((Chip { rev, hw }, pcie_cap))
}
