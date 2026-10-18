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

//! What a bring-up holds from the broker, each given back when dropped:
//! the claim, the register window, and the DMA regions.

use nonos_libc::{mk_device_release, mk_dma_map, mk_dma_unmap, mk_mmio_unmap, DmaMapOut};

use super::say::refused;
use crate::ufs::env::DmaBuf;

/// The claim and the register window. Dropping it unmaps the window and
/// releases the claim, and the broker stops bus mastering on release.
pub struct Claim {
    pub device_id: u64,
    pub epoch: u64,
    pub mmio_grant: Option<u64>,
}

impl Drop for Claim {
    fn drop(&mut self) {
        if let Some(g) = self.mmio_grant {
            let _ = mk_mmio_unmap(g);
        }
        let _ = mk_device_release(self.device_id);
    }
}

/// A zeroed DMA region the claimed host may reach, unmapped when dropped.
pub struct DmaRegion {
    grant_id: u64,
    pub buf: DmaBuf,
}

impl DmaRegion {
    pub fn map(c: &Claim, length: u64, what: &[u8]) -> Option<Self> {
        let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
        let r = mk_dma_map(c.device_id, c.epoch, length, 0, &mut out);
        if r < 0 {
            refused(what, r);
            return None;
        }
        let buf = DmaBuf { va: out.user_va, bus: out.device_addr, len: out.length as usize };
        Some(Self { grant_id: out.grant_id, buf })
    }
}

impl Drop for DmaRegion {
    fn drop(&mut self) {
        let _ = mk_dma_unmap(self.grant_id);
    }
}
