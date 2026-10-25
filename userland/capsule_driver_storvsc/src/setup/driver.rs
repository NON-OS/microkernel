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

//! What the capsule holds for its life: the units it serves over the
//! channel, and what it gives back if it leaves.

use nonos_libc::{mk_device_release, mk_dma_unmap};
use nonos_scsi_serve::Units;
use nonos_vmbus::Pages;

use crate::hba::VmHba;
use crate::host::Kernel;

pub type StorHba = VmHba<Kernel, Pages>;

pub struct Driver {
    pub units: Units<StorHba>,
    pub device_id: u64,
    pub dma_grant: u64,
}

impl Driver {
    /// Close the channel and take its pages back from the host, then give
    /// back the grant and the claim. A host that does not confirm the pages
    /// gone leaves them to the kernel, which keeps them until it does.
    pub fn release(&mut self) {
        let h = &mut self.units.hba;
        let _ = h.chan.close(&mut h.host);
        abandon(self.device_id, self.dma_grant);
    }
}

/// The grant and the claim given back.
pub fn abandon(device_id: u64, dma_grant: u64) {
    let _ = mk_dma_unmap(dma_grant);
    let _ = mk_device_release(device_id);
}
