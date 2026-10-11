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

use super::constants::GICD_CTLR;
use super::device::GicDistributor;
use crate::arch::aarch64::gic::wait::until;

/// `GICD_CTLR.RWP`: a write to `GICD_CTLR` or `GICD_ICENABLER` is still in
/// flight. GICv2 has no such bit and reads it as zero.
const CTLR_RWP: u32 = 1 << 31;

impl GicDistributor {
    /// Wait for the last control or disable write to take effect.
    pub(super) fn settle(&self) -> Result<(), &'static str> {
        if until(|| self.read_reg(GICD_CTLR) & CTLR_RWP == 0) {
            Ok(())
        } else {
            Err("GIC distributor register write never completed")
        }
    }
}
