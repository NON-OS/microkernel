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

use nonos_libc::{mk_device_release, mk_mmio_unmap};

use crate::dispi::{restore, Dispi};
use crate::regs::Regs;

/// What one bring-up attempt holds of the broker, and the scanout firmware
/// left, given back in that order: the DISPI state written back while the
/// registers are still mapped, both mappings, and the claim last.
pub struct BrokerHandles {
    device_id: u64,
    reg_grant_id: u64,
    fb_grant_id: u64,
    firmware: Option<(Regs, Dispi)>,
}

impl BrokerHandles {
    pub const fn new(device_id: u64, reg_grant_id: u64, fb_grant_id: u64) -> Self {
        Self { device_id, reg_grant_id, fb_grant_id, firmware: None }
    }

    /// The state firmware left in `regs`, read before the driver writes
    /// any of it, written back when the handles drop. A capsule killed
    /// outright drops nothing: where the driver kept OVMF's mode it wrote no
    /// DISPI register, so that mode stays, and a mode it set stays set.
    /// Neither brings the kernel's GOP blit back: the claim of an adapter
    /// whose BAR0 holds the boot framebuffer stopped it for the boot.
    pub fn restore_at_drop(&mut self, regs: Regs, firmware: Dispi) {
        self.firmware = Some((regs, firmware));
    }

    /// The broker's id of the adapter, which a DGON names when it leaves.
    pub fn device_id(&self) -> u64 {
        self.device_id
    }
}

impl Drop for BrokerHandles {
    fn drop(&mut self) {
        if let Some((regs, firmware)) = self.firmware {
            restore(regs, &firmware);
        }
        let _ = mk_mmio_unmap(self.fb_grant_id);
        let _ = mk_mmio_unmap(self.reg_grant_id);
        let _ = mk_device_release(self.device_id);
    }
}
