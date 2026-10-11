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

//! DMA phase. Four grants per setup: RX ring, RX buffer pool, TX ring, TX
//! buffer pool. Each step rolls back every prior grant in reverse on failure
//! so the broker never holds a partial setup. Grants are page aligned, which
//! covers the 128-byte ring base alignment the queues need.

use nonos_libc::{mk_dma_map, DmaMapOut, MmioMapOut};

use crate::constants::queue::{
    RX_BUFFER_POOL_BYTES, RX_RING_BYTES, TX_BUFFER_POOL_BYTES, TX_RING_BYTES,
};

use super::rollback;

const PAGE_MASK: u64 = 0xFFF;

fn alloc(device_id: u64, claim_epoch: u64, bytes: usize) -> Option<DmaMapOut> {
    let mut out = DmaMapOut { user_va: 0, device_addr: 0, length: 0, grant_id: 0 };
    let len = (bytes as u64 + PAGE_MASK) & !PAGE_MASK;
    if mk_dma_map(device_id, claim_epoch, len, 0, &mut out) < 0 {
        None
    } else {
        Some(out)
    }
}

pub type Grants = (DmaMapOut, DmaMapOut, DmaMapOut, DmaMapOut);

pub fn map_rings_and_buffers(
    device_id: u64,
    claim_epoch: u64,
    mmio: &MmioMapOut,
) -> Result<Grants, &'static str> {
    let rx_ring = alloc(device_id, claim_epoch, RX_RING_BYTES).ok_or_else(|| {
        rollback::after(device_id, mmio, &[]);
        "dma map refused (rx ring)"
    })?;
    let rx_buf = alloc(device_id, claim_epoch, RX_BUFFER_POOL_BYTES).ok_or_else(|| {
        rollback::after(device_id, mmio, &[rx_ring.grant_id]);
        "dma map refused (rx buffers)"
    })?;
    let tx_ring = alloc(device_id, claim_epoch, TX_RING_BYTES).ok_or_else(|| {
        rollback::after(device_id, mmio, &[rx_ring.grant_id, rx_buf.grant_id]);
        "dma map refused (tx ring)"
    })?;
    let tx_buf = alloc(device_id, claim_epoch, TX_BUFFER_POOL_BYTES).ok_or_else(|| {
        rollback::after(device_id, mmio, &[rx_ring.grant_id, rx_buf.grant_id, tx_ring.grant_id]);
        "dma map refused (tx buffers)"
    })?;
    Ok((rx_ring, rx_buf, tx_ring, tx_buf))
}
