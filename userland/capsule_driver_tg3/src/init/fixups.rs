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

//! tg3_reset_hw's 57765-class block after the chip reset: on A0, keep the
//! PCI Express L2 clock request off; on every part, the transmit-hang fix
//! (constants/chip.rs CHIPREV_57765_AX says why every part); then the MAC
//! clock at 6.25 MHz while the link runs at 10 Mb/s. The CPMU is present,
//! so tg3 leaves the PCI clock control alone.

use crate::constants::chip::{chip_rev, CHIPREV_57765_AX, CHIPREV_ID_57765_A0};
use crate::constants::cpmu::*;
use crate::constants::grc::GRC_MODE_PCIE_PORT_MASK;
use crate::constants::grc::{GRC_MODE, GRC_MODE_PCIE_DL_SEL, GRC_MODE_PCIE_PL_SEL};
use crate::regs::Regs;

/*
 * SAFETY for every access: `regs` carries the broker MmioMap base for BAR0,
 * and each offset (GRC_MODE, the CPMU registers, the PCI Express window at
 * TG3_PCIE_TLDLPL_PORT) is a 32-bit register inside it.
 */
pub fn run(regs: &Regs, rev: u32) {
    if rev == CHIPREV_ID_57765_A0 {
        through_port(regs, GRC_MODE_PCIE_PL_SEL, |r| unsafe {
            let at = TG3_PCIE_TLDLPL_PORT + TG3_PCIE_PL_LO_PHYCTL5;
            r.w32(at, r.r32(at) | TG3_PCIE_PL_LO_PHYCTL5_DIS_L2CLKREQ);
        });
    }
    if chip_rev(rev) != CHIPREV_57765_AX {
        unsafe {
            let v = regs.r32(TG3_CPMU_PADRNG_CTL);
            regs.w32(TG3_CPMU_PADRNG_CTL, v | TG3_CPMU_PADRNG_CTL_RDIV2);
        }
        through_port(regs, GRC_MODE_PCIE_DL_SEL, |r| unsafe {
            let at = TG3_PCIE_TLDLPL_PORT + TG3_PCIE_DL_LO_FTSMAX;
            let v = r.r32(at) & !TG3_PCIE_DL_LO_FTSMAX_MSK;
            r.w32(at, v | TG3_PCIE_DL_LO_FTSMAX_VAL);
        });
    }
    unsafe {
        let v = regs.r32(TG3_CPMU_LSPD_10MB_CLK) & !CPMU_LSPD_10MB_MACCLK_MASK;
        regs.w32(TG3_CPMU_LSPD_10MB_CLK, v | CPMU_LSPD_10MB_MACCLK_6_25);
    }
}

/// The window at TG3_PCIE_TLDLPL_PORT shows the block GRC_MODE's port bits
/// choose; `port` is chosen for `f`, and GRC_MODE is put back after.
fn through_port(regs: &Regs, port: u32, f: impl FnOnce(&Regs)) {
    unsafe {
        let grc_mode = regs.r32(GRC_MODE);
        regs.w32(GRC_MODE, (grc_mode & !GRC_MODE_PCIE_PORT_MASK) | port);
        f(regs);
        regs.w32(GRC_MODE, grc_mode);
    }
}
