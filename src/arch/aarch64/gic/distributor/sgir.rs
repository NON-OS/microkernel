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

use core::arch::asm;

use super::device::GicDistributor;

/// GICv2's software-generated interrupt register. GICv3 raises SGIs through
/// `ICC_SGI1R_EL1` instead.
const GICD_SGIR: u64 = 0x0F00;

impl GicDistributor {
    pub(in crate::arch::aarch64::gic) fn write_sgir(&self, value: u32) {
        // The target must see everything written before it is interrupted.
        // SAFETY: a barrier only; it touches no memory and no register state.
        unsafe { asm!("dsb ishst", options(nostack, preserves_flags)) };
        self.write_reg(GICD_SGIR, value);
    }
}
