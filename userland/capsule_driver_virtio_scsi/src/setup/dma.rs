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

//! The one DMA grant: the ring, the request and its response, and the data
//! buffer, coherent, as the device reads and writes them while it runs.
//! virtio addresses are 64-bit: the grant may sit anywhere.

use nonos_libc::{mk_dma_map, mk_dma_unmap, DmaMapOut, MK_DMA_MAP_COHERENT};

use crate::constants::layout::GRANT_BYTES;

pub fn map(device_id: u64, epoch: u64) -> Result<DmaMapOut, &'static str> {
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    if mk_dma_map(device_id, epoch, GRANT_BYTES, MK_DMA_MAP_COHERENT, &mut out) < 0 {
        return Err("dma map failed");
    }
    if out.length < GRANT_BYTES {
        let _ = mk_dma_unmap(out.grant_id);
        return Err("dma grant short");
    }
    Ok(out)
}
