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

//! The claimed function as the shared transport reaches it: the kernel's
//! hardware broker, one call each.

use nonos_libc::{mk_mmio_map, mk_mmio_unmap, mk_pci_config_read, MmioMapOut};
use nonos_virtio::{Broker, MmioGrant};

pub struct LibcBroker {
    device_id: u64,
    claim_epoch: u64,
}

impl LibcBroker {
    pub const fn new(device_id: u64, claim_epoch: u64) -> Self {
        Self { device_id, claim_epoch }
    }
}

// SAFETY: MkMmioMap installs `length` bytes at `user_va` in this address
// space, readable and writable, and they stay until MkMmioUnmap or the
// claim's release; the driver drops its window on both.
unsafe impl Broker for LibcBroker {
    fn config_read32(&mut self, offset: u32) -> Option<u32> {
        let rc = mk_pci_config_read(self.device_id, self.claim_epoch, offset, 4);
        if rc < 0 {
            return None;
        }
        Some(rc as u32)
    }

    fn mmio_map(&mut self, bar: u8, offset: u64, length: u64) -> Result<MmioGrant, i64> {
        let mut out = MmioMapOut { user_va: 0, length: 0, grant_id: 0 };
        let rc =
            mk_mmio_map(self.device_id, self.claim_epoch, bar as u32, 0, offset, length, &mut out);
        if rc < 0 {
            return Err(rc);
        }
        Ok(MmioGrant { user_va: out.user_va, length: out.length, grant_id: out.grant_id })
    }

    fn mmio_unmap(&mut self, grant_id: u64) -> bool {
        mk_mmio_unmap(grant_id) >= 0
    }
}
