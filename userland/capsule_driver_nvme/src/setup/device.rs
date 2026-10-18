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

use crate::admin::{enable, reset_to_disabled};
use crate::constants::{REG_CSTS, REG_INTMS};
use crate::controller::ControllerInfo;
use crate::error::NvmeResult;
use crate::handles::BrokerHandles;
use crate::regs::Regs;

/// A claimed controller whose registers read as NVMe's. Dropping it is the
/// one teardown every path takes (C2): the controller disabled while its
/// queues are still mapped, then the handles' drop releases the device,
/// which clears Bus Master, reads it back and only then drains every grant;
/// no queue memory is unmapped before that (dma::DmaRegion).
pub struct Device {
    pub handles: BrokerHandles,
    pub regs: Regs,
    /// The registers as found at bring-up, for CAP.TO and the stride.
    pub info: ControllerInfo,
    /// CC.EN may be set, by firmware or the driver's own enable.
    enabled: bool,
}

impl Device {
    pub const fn new(handles: BrokerHandles, regs: Regs, info: ControllerInfo) -> Self {
        Self { handles, regs, info, enabled: true }
    }

    /// CSTS as it reads now; all ones when nothing answers at BAR0.
    pub fn csts(&self) -> u32 {
        // SAFETY: CSTS lies in the register block bring-up checked.
        unsafe { self.regs.r32(REG_CSTS) }
    }

    /// Clear CC.EN and wait, bounded by CAP.TO, for RDY to follow. One that
    /// fails is not tried again on the way out: the release stops it.
    pub fn disable(&mut self) -> NvmeResult<()> {
        self.enabled = false;
        reset_to_disabled(self.regs, self.info)
    }

    /// Set CC.EN and wait for RDY. The reset cleared INTMS: without MSI-X,
    /// mask what nothing services (INTMS is off limits under MSI-X, 3.1.4).
    pub fn enable(&mut self) -> NvmeResult<()> {
        self.enabled = true;
        enable(self.regs, self.info)?;
        if self.handles.irq_grant_id() == 0 {
            // SAFETY: INTMS lies in the register block bring-up checked.
            unsafe { self.regs.w32(REG_INTMS, u32::MAX) };
        }
        Ok(())
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        if self.enabled {
            let _ = self.disable();
        }
    }
}
