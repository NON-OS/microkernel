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

//! What only a GICv2 distributor does: SPI targets by CPU interface mask and
//! banked registers for SGIs and PPIs.

use super::constants::{
    GICD_CTLR, GICD_ICENABLER, GICD_ICPENDR, GICD_IPRIORITYR, GICD_ISENABLER, GICD_ITARGETSR,
};
use super::device::GicDistributor;

/// Bit 0 enables forwarding: group 1 from the non-secure side, group 0 on a
/// GIC without security. Groups are left as reset or firmware set them,
/// matching what the CPU interface enables.
const CTLR_ENABLE: u32 = 1;

impl GicDistributor {
    /// Every SPI disabled, unpending, at the default priority and aimed at
    /// `boot_mask`. A GIC built for one CPU reads targets as zero and ignores
    /// the writes, which is right: there is nowhere else to send them.
    pub(in crate::arch::aarch64::gic) fn init_v2(&self, boot_mask: u8) {
        self.write_reg(GICD_CTLR, 0);
        let num_irqs = self.num_irqs();
        for irq in (32..num_irqs).step_by(32) {
            self.write_reg(GICD_ICENABLER + (irq / 32) as u64 * 4, 0xFFFF_FFFF);
            self.write_reg(GICD_ICPENDR + (irq / 32) as u64 * 4, 0xFFFF_FFFF);
        }
        let targets = u32::from_ne_bytes([boot_mask; 4]);
        for irq in (32..num_irqs).step_by(4) {
            self.write_reg(GICD_IPRIORITYR + irq as u64, 0xA0A0_A0A0);
            self.write_reg(GICD_ITARGETSR + irq as u64, targets);
        }
        self.write_reg(GICD_CTLR, CTLR_ENABLE);
    }

    /// The calling CPU's SGIs on and PPIs off, all at the default priority.
    /// These registers are banked, so each CPU does this for itself.
    pub(in crate::arch::aarch64::gic) fn init_banked_v2(&self) {
        self.write_reg(GICD_ICENABLER, 0xFFFF_0000);
        self.write_reg(GICD_ICPENDR, 0xFFFF_0000);
        for irq in (0..32u64).step_by(4) {
            self.write_reg(GICD_IPRIORITYR + irq, 0xA0A0_A0A0);
        }
        self.write_reg(GICD_ISENABLER, 0x0000_FFFF);
    }

    /// The calling CPU's interface mask. The first eight target registers are
    /// banked and read back that CPU's own bit for every SGI and PPI.
    pub(in crate::arch::aarch64::gic) fn this_cpu_mask(&self) -> u8 {
        let word = (0..8u64).fold(0, |acc, i| acc | self.read_reg(GICD_ITARGETSR + i * 4));
        word.to_ne_bytes().iter().fold(0, |acc, byte| acc | byte)
    }
}
