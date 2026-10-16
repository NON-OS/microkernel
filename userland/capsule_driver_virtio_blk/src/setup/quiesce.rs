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

//! Stopping the device and giving it back (plan/50 C2), before a fresh
//! bring-up. The reset takes its queue down, a request that timed out
//! included; bus mastering goes off (`pci`: defence in depth until K6);
//! MkDeviceRelease, last, clears bus mastering itself and only then tears
//! down the DMA, interrupt and register grants. Nothing is unmapped here,
//! so no buffer is freed while the device can still reach it.

use nonos_libc::mk_device_release;

use super::driver::Driver;
use super::pci;

impl Driver {
    /// True when the device read back reset and the broker took it back.
    pub fn quiesce(&self) -> bool {
        let stopped = self.transport.reset();
        let _ = pci::command(self.device_id, self.claim_epoch, pci::DECODE);
        mk_device_release(self.device_id) >= 0 && stopped
    }
}
