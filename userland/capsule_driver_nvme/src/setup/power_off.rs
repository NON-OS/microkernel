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

//! The controller's part of the power-off (server::handlers::power_off).

use super::device::Device;
use crate::admin::shut_down;
use crate::error::NvmeResult;

impl Device {
    /// The power-off: the shutdown notification and its bounded wait, then the
    /// reset, whichever way the wait ended, so the controller is disabled
    /// before power goes. The shutdown's own failure is the one returned.
    pub fn power_off(&mut self, rtd3e_us: u32) -> NvmeResult<()> {
        let shut = shut_down(self.regs, rtd3e_us);
        let reset = self.disable();
        shut.and(reset)
    }
}
