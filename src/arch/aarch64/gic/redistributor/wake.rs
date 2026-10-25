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

use super::constants::{GICR_CTLR, GICR_WAKER, WAKER_CHILDREN_ASLEEP, WAKER_PROCESSOR_SLEEP};
use super::device::GicRedistributor;
use crate::arch::aarch64::gic::wait::until;

/// `GICR_CTLR.RWP`: a write to `GICR_ICENABLER0` has not taken effect yet.
const CTLR_RWP: u32 = 1 << 3;

impl GicRedistributor {
    /// Tell the frame its CPU is awake and wait until it agrees. Until then
    /// the frame drops this CPU's SGIs and PPIs on the floor.
    pub(super) fn wake(&self) -> Result<(), &'static str> {
        let waker = self.read_reg(GICR_WAKER);
        if waker & WAKER_PROCESSOR_SLEEP != 0 {
            self.write_reg(GICR_WAKER, waker & !WAKER_PROCESSOR_SLEEP);
        }
        if until(|| self.read_reg(GICR_WAKER) & WAKER_CHILDREN_ASLEEP == 0) {
            Ok(())
        } else {
            Err("GICv3 redistributor stayed asleep")
        }
    }

    /// Wait for disables written to this frame to take effect.
    pub(super) fn settle(&self) -> Result<(), &'static str> {
        if until(|| self.read_reg(GICR_CTLR) & CTLR_RWP == 0) {
            Ok(())
        } else {
            Err("GICv3 redistributor register write never completed")
        }
    }
}
