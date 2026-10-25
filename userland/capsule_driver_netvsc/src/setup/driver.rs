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

//! What the capsule holds for its life: the adapter it serves, and what it
//! gives back if it leaves.

use nonos_libc::{mk_device_release, mk_dma_unmap};

use crate::nic::Net;

pub struct Driver {
    pub net: Net,
    pub device_id: u64,
    pub grants: [u64; 3],
    pub relid: u32,
    /// The receive and send buffers' GPADLs, beside the rings'.
    pub buffers: [u32; 2],
}

impl Driver {
    /// Close the channel and take its GPADLs back, then give back the
    /// grants and the claim. A host that does not confirm a GPADL gone
    /// leaves its pages to the kernel, which keeps them until it does.
    pub fn release(&mut self) {
        let l = &mut self.net.link;
        let _ = l.chan.close(&mut l.host);
        for id in self.buffers {
            let _ = nonos_vmbus::teardown(&mut l.host, self.relid, id);
        }
        abandon(self.device_id, &self.grants);
    }
}

/// The grants made so far and the claim given back.
pub fn abandon(device_id: u64, grants: &[u64]) {
    for g in grants.iter().filter(|g| **g != 0) {
        let _ = mk_dma_unmap(*g);
    }
    let _ = mk_device_release(device_id);
}
