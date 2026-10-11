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

//! Device register accesses with the ordering a driver needs: a store after
//! every earlier store to DMA memory, as the doorbell that announces it must
//! be; a load before any later access to DMA memory, as the status that
//! vouches for it must be. One aligned access each, never split or merged.
//!
//! # Safety
//! For all of these: `addr` is a register in a window the broker mapped for
//! this process, aligned to the access size.

use core::ptr::{read_volatile, write_volatile};

use super::dma_order::{dma_rmb, dma_wmb};

macro_rules! access {
    ($read:ident, $write:ident, $ty:ty) => {
        /// # Safety
        /// As the module says.
        #[inline(always)]
        pub unsafe fn $read(addr: usize) -> $ty {
            // SAFETY: the caller's mapped, aligned register.
            let value = unsafe { read_volatile(addr as *const $ty) };
            dma_rmb();
            value
        }

        /// # Safety
        /// As the module says.
        #[inline(always)]
        pub unsafe fn $write(addr: usize, value: $ty) {
            dma_wmb();
            // SAFETY: the caller's mapped, aligned register.
            unsafe { write_volatile(addr as *mut $ty, value) }
        }
    };
}

access!(mmio_read8, mmio_write8, u8);
access!(mmio_read16, mmio_write16, u16);
access!(mmio_read32, mmio_write32, u32);
access!(mmio_read64, mmio_write64, u64);
