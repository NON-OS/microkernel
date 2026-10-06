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

//! A broker DMA region, unmapped when dropped.

use nonos_libc::{mk_dma_map, mk_dma_unmap, DmaMapOut};

use super::super::env::DmaBuf;
use super::super::error::{EmmcError, EmmcResult};

pub struct DmaRegion {
    grant_id: u64,
    buf: DmaBuf,
}

impl DmaRegion {
    /// A zeroed, physically contiguous region of `length` bytes (a multiple
    /// of 4 KiB) the claimed device may reach.
    pub fn map(device_id: u64, claim_epoch: u64, length: u64) -> EmmcResult<Self> {
        let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
        let r = mk_dma_map(device_id, claim_epoch, length, 0, &mut out);
        if r < 0 {
            return Err(EmmcError::Broker(r));
        }
        let buf = DmaBuf { va: out.user_va, bus: out.device_addr, len: out.length as usize };
        Ok(Self { grant_id: out.grant_id, buf })
    }

    pub const fn buf(&self) -> DmaBuf {
        self.buf
    }
}

impl Drop for DmaRegion {
    fn drop(&mut self) {
        let _ = mk_dma_unmap(self.grant_id);
    }
}
