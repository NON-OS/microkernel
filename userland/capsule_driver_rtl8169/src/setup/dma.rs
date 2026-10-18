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

use nonos_libc::{mk_dma_map, DmaMapOut, MmioMapOut, MK_DMA_MAP_DMA32};

use crate::chip::MacVersion;
use crate::constants::queue::{RX_BUFFER_BYTES, RX_RING_BYTES, TX_BUFFER_BYTES, TX_RING_BYTES};

use super::rollback;

const PAGE_MASK: u64 = 0xFFF;

fn page_round(n: u64) -> u64 {
    (n + PAGE_MASK) & !PAGE_MASK
}

fn alloc(device_id: u64, epoch: u64, bytes: u64, flags: u32) -> Option<DmaMapOut> {
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    let r = mk_dma_map(device_id, epoch, page_round(bytes), flags, &mut out);
    if r < 0 {
        None
    } else {
        Some(out)
    }
}

pub fn map_all(
    device_id: u64,
    epoch: u64,
    mmio: &MmioMapOut,
    ver: MacVersion,
) -> Result<(DmaMapOut, DmaMapOut, DmaMapOut, DmaMapOut), &'static str> {
    let flags = if ver.dma32_only() { MK_DMA_MAP_DMA32 } else { 0 };
    let rx_ring = alloc(device_id, epoch, RX_RING_BYTES as u64, flags).ok_or_else(|| {
        rollback::after(device_id, epoch, mmio, ver);
        "rx ring dma failed"
    })?;
    let rx_buf = alloc(device_id, epoch, RX_BUFFER_BYTES as u64, flags).ok_or_else(|| {
        rollback::after(device_id, epoch, mmio, ver);
        "rx buffer dma failed"
    })?;
    let tx_ring = alloc(device_id, epoch, TX_RING_BYTES as u64, flags).ok_or_else(|| {
        rollback::after(device_id, epoch, mmio, ver);
        "tx ring dma failed"
    })?;
    let tx_buf = alloc(device_id, epoch, TX_BUFFER_BYTES as u64, flags).ok_or_else(|| {
        rollback::after(device_id, epoch, mmio, ver);
        "tx buffer dma failed"
    })?;
    Ok((rx_ring, rx_buf, tx_ring, tx_buf))
}
