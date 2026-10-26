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

//! The one coherent DMA grant on the record, for the rings and the data
//! buffer, refused when it comes back short or off a page boundary.

use nonos_libc::{mk_device_release, mk_dma_map, DmaMapOut, MK_DMA_MAP_COHERENT};

use super::driver::abandon;
use crate::constants::layout::{GRANT_BYTES, PAGE};

/// The grant, or the claim given back with whatever was mapped.
pub fn map(device_id: u64, epoch: u64) -> Result<DmaMapOut, &'static str> {
    let mut dma = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    if mk_dma_map(device_id, epoch, GRANT_BYTES, MK_DMA_MAP_COHERENT, &mut dma) < 0 {
        let _ = mk_device_release(device_id);
        return Err("dma map failed");
    }
    if dma.length < GRANT_BYTES || !dma.device_addr.is_multiple_of(PAGE as u64) {
        abandon(device_id, dma.grant_id);
        return Err("dma grant short");
    }
    Ok(dma)
}
