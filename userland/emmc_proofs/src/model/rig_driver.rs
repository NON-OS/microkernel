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

//! The driver's host and disk on a rig.

use super::host::SimBus;
use super::rig::Rig;
use super::test_env::{TestClock, TestDisk, TestHost};
use crate::emmc::disk::EmmcDisk;
use crate::emmc::error::EmmcResult;
use crate::emmc::sdhci::Host;

impl Rig {
    pub fn host(&self, intel_emmc: bool) -> TestHost {
        Host::new(
            SimBus(self.sim.clone()),
            TestClock(self.now.clone()),
            self.log.clone(),
            self.table,
            intel_emmc,
        )
    }

    /// The driver's host, reset as every open readies it (`EmmcDisk::reset`).
    pub fn reset(&self, intel_emmc: bool) -> TestHost {
        EmmcDisk::reset(self.host(intel_emmc)).expect("host reset")
    }

    pub fn disk(&self, intel_emmc: bool) -> EmmcResult<TestDisk> {
        EmmcDisk::reset(self.host(intel_emmc)).and_then(|h| EmmcDisk::bring_up(h, self.data))
    }
}
