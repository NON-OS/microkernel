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

//! A DMA buffer below 4 GiB, mapped uncached so neither side needs a flush,
//! as Linux takes it from dma_alloc_coherent with a 32-bit mask.

use nonos_libc::{mk_dma_map, mk_dma_unmap, DmaMapOut, MK_DMA_MAP_COHERENT, MK_DMA_MAP_DMA32};

use crate::error::{Result, RtsxError};

pub struct Region {
    grant_id: u64,
    user_va: u64,
    device_addr: u32,
    len: u64,
}

impl Region {
    pub fn map(device_id: u64, claim_epoch: u64, len: u64) -> Result<Self> {
        let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
        let flags = MK_DMA_MAP_DMA32 | MK_DMA_MAP_COHERENT;
        if mk_dma_map(device_id, claim_epoch, len, flags, &mut out) < 0 {
            return Err(RtsxError::DmaMap);
        }
        let region = Self { grant_id: out.grant_id, user_va: out.user_va, device_addr: 0, len };
        let end = out.device_addr.checked_add(len).ok_or(RtsxError::DmaAbove4G)?;
        if end > 1 << 32 || out.length < len {
            return Err(RtsxError::DmaAbove4G);
        }
        Ok(Self { device_addr: out.device_addr as u32, ..region })
    }

    pub const fn device_addr(&self) -> u32 {
        self.device_addr
    }

    pub fn write_u32(&self, offset: u64, value: u32) {
        if offset + 4 <= self.len {
            // SAFETY: inside the mapping the broker returned (checked
            // against `len`), which lives until this Region is dropped.
            unsafe { core::ptr::write_volatile((self.user_va + offset) as *mut u32, value) }
        }
    }

    /// Zero past the end, which no caller asks for.
    pub fn read_u8(&self, offset: u64) -> u8 {
        if offset >= self.len {
            return 0;
        }
        // SAFETY: as write_u32.
        unsafe { core::ptr::read_volatile((self.user_va + offset) as *const u8) }
    }
}

impl Drop for Region {
    fn drop(&mut self) {
        let _ = mk_dma_unmap(self.grant_id);
    }
}
