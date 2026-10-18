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

use nonos_libc::{mk_device_release, mk_irq_unbind, mk_mmio_unmap};

use super::broker_handles::BrokerHandles;
use crate::controller::quiesce::quiesce;
use crate::error::reason;
use crate::regs::Regs;
use crate::setup::Line;

/// The controller is stopped and held in reset, bus mastering is switched
/// off, the interrupt and the register window go, and only then the claim.
/// No DMA grant is unmapped here: the release gives them back (the rings,
/// and on a playing controller the BDL and PCM ring too), and the broker
/// clears bus mastering itself and reads it back before it does. The DMA
/// grants used to be unmapped first, and an unmap frees its frames at once,
/// under a RIRB engine still taking answers into them.
impl Drop for BrokerHandles {
    fn drop(&mut self) {
        if let Err(e) = quiesce(Regs::new(self.mmio_user_va)) {
            Line::new("[HDA] teardown: ").s(reason(e)).emit();
        }
        self.bus_master_off();
        let _ = mk_irq_unbind(self.irq_grant_id);
        let _ = mk_mmio_unmap(self.mmio_grant_id);
        let _ = mk_device_release(self.device_id);
    }
}
