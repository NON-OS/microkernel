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

//! What the broker granted, given back when an attempt fails or the
//! driver ends: BAR0's grant, then the claim. The scanout surfaces' grants
//! are `DmaFrames`', given back before these.

use nonos_libc::{mk_device_release, mk_mmio_unmap};

pub struct BrokerHandles {
    pub device_id: u64,
    pub epoch: u64,
    pub bar0: Option<u64>,
}

impl BrokerHandles {
    pub const fn new(device_id: u64, epoch: u64) -> Self {
        BrokerHandles { device_id, epoch, bar0: None }
    }
}

impl Drop for BrokerHandles {
    fn drop(&mut self) {
        if let Some(g) = self.bar0 {
            let _ = mk_mmio_unmap(g);
        }
        let _ = mk_device_release(self.device_id);
    }
}
