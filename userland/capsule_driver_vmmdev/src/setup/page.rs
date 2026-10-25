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

//! The request page: one coherent DMA page whose whole span the request
//! port's 32-bit address can name. A grant placed higher, or short, is
//! given back.

use nonos_libc::{mk_dma_map, mk_dma_unmap, DmaMapOut, MK_DMA_MAP_COHERENT, MK_DMA_MAP_DMA32};

pub const PAGE: u64 = 0x1000;

pub fn request_page(device_id: u64, epoch: u64) -> Option<DmaMapOut> {
    let mut page = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    let flags = MK_DMA_MAP_DMA32 | MK_DMA_MAP_COHERENT;
    if mk_dma_map(device_id, epoch, PAGE, flags, &mut page) < 0 {
        return None;
    }
    if page.length < PAGE || page.device_addr.saturating_add(PAGE) > 1 << 32 {
        let _ = mk_dma_unmap(page.grant_id);
        return None;
    }
    Some(page)
}
