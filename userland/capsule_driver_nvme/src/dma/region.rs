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

use nonos_libc::{mk_dma_map, mk_dma_unmap, DmaMapOut};

use crate::error::{NvmeError, NvmeResult};

/// The page every grant must start on: queues, PRP entries and host memory
/// buffer descriptors all name whole 4 KiB pages.
const PAGE_MASK: u64 = 0xfff;

/// Memory the controller may be given. Dropping a region unmaps nothing:
/// memory the controller may have seen goes back only with the device's
/// release, which turns bus mastering off before it drains the grants, so
/// no frame is freed while the device can still write it (C2). A failed
/// disable, a command never answered, or a drop order cannot change that.
/// `unmap` gives back memory the controller never took.
pub struct DmaRegion {
    grant_id: u64,
    user_va: u64,
    device_addr: u64,
}

impl DmaRegion {
    pub fn map(device_id: u64, claim_epoch: u64, length: u64) -> NvmeResult<Self> {
        let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
        let r = mk_dma_map(device_id, claim_epoch, length, 0, &mut out);
        if r < 0 {
            return Err(NvmeError::BrokerCallFailed);
        }
        // A short grant would let the controller write past what is ours; a
        // misaligned one would be named by addresses it does not start at.
        // Neither has been handed to the controller, so it goes back now.
        if out.length < length || (out.user_va | out.device_addr) & PAGE_MASK != 0 {
            let _ = mk_dma_unmap(out.grant_id);
            return Err(NvmeError::DmaGrantShort);
        }
        Ok(Self { grant_id: out.grant_id, user_va: out.user_va, device_addr: out.device_addr })
    }

    pub const fn user_va(&self) -> u64 {
        self.user_va
    }

    pub const fn device_addr(&self) -> u64 {
        self.device_addr
    }

    /// Unmap now. Only for memory whose address no command has carried, or
    /// that the controller refused.
    pub fn unmap(self) {
        let _ = mk_dma_unmap(self.grant_id);
    }
}
