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

//! What one bring-up attempt holds from the broker once every grant is in,
//! and the chip init that either keeps it or gives all of it back.
//!
//! The init used to fail with the claim, the register mapping, the line and
//! the staging DMA all still held; a retry was then refused its own leftover
//! claim, and only the process exit gave them back.

use nonos_libc::{mk_device_release, mk_irq_unbind, mk_mmio_unmap};

use super::command::command;
use crate::init::bring_up;
use crate::regs::Regs;

/// Each grant is held until it is given back, then gone: a handle given back
/// may name a newer grant of this process, so it is never given back twice.
pub struct Grants {
    pub device_id: u64,
    /// The claim's epoch while the claim is held.
    pub claim: Option<u64>,
    pub mmio: Option<u64>,
    pub irq: Option<u64>,
}

impl Grants {
    /// Bus Master off first, so the device reaches no memory of ours,
    /// then every grant in the reverse of the order it was taken, then the
    /// claim, which on its own tears the rest down as well. Each is taken
    /// as it goes, so a second release sends nothing.
    pub fn release(&mut self) {
        if let Some(epoch) = self.claim {
            let _ = command(self.device_id, epoch, false);
        }
        if let Some(irq) = self.irq.take() {
            let _ = mk_irq_unbind(irq);
        }
        if let Some(mmio) = self.mmio.take() {
            let _ = mk_mmio_unmap(mmio);
        }
        if self.claim.take().is_some() {
            let _ = mk_device_release(self.device_id);
        }
    }
}

/// Wake the chip on an attempt's grants, or give all of them back. Bus
/// Master stays off: it goes on only right before the firmware boots, once
/// the device's own master read back stopped (`gen3::outcome::master_on`).
pub fn init_or_release(regs: Regs, grants: &mut Grants) -> Result<(), &'static str> {
    bring_up(regs).inspect_err(|_| grants.release())
}
