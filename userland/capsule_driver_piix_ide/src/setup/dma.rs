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

//! The one DMA grant: the descriptor table and the transfer buffer,
//! coherent, below 4 GiB. The table pointer and every descriptor carry
//! 32-bit bus addresses, so a grant the broker placed higher, or one shorter
//! than asked, is given back.

use nonos_libc::{mk_dma_map, mk_dma_unmap, DmaMapOut, MK_DMA_MAP_COHERENT, MK_DMA_MAP_DMA32};

use crate::constants::buffer::GRANT_BYTES;

const DMA32_CEILING: u64 = 1 << 32;

pub fn map(device_id: u64, epoch: u64) -> Option<DmaMapOut> {
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    let flags = MK_DMA_MAP_DMA32 | MK_DMA_MAP_COHERENT;
    if mk_dma_map(device_id, epoch, GRANT_BYTES, flags, &mut out) < 0 {
        return None;
    }
    if out.length < GRANT_BYTES || !fits_32(out.device_addr, out.length) {
        let _ = mk_dma_unmap(out.grant_id);
        return None;
    }
    Some(out)
}

/// Whether `[addr, addr + len)` is nameable in 32 bits.
pub const fn fits_32(addr: u64, len: u64) -> bool {
    match addr.checked_add(len) {
        Some(end) => end <= DMA32_CEILING,
        None => false,
    }
}
