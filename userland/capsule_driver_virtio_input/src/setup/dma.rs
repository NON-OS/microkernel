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

//! The one DMA grant: the event queue's ring and its buffers, coherent, as
//! the device reads the rings and writes the buffers while it runs.

use nonos_libc::{mk_dma_map, DmaMapOut, MK_DMA_MAP_COHERENT};

use crate::constants::queue::RING_BYTES;

pub fn map(device_id: u64, epoch: u64) -> Result<DmaMapOut, &'static str> {
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    let length = RING_BYTES as u64;
    if mk_dma_map(device_id, epoch, length, MK_DMA_MAP_COHERENT, &mut out) < 0 {
        return Err("dma map failed (event queue)");
    }
    Ok(out)
}
