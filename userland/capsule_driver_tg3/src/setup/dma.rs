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

//! DMA phase. Three grants per setup: the rings with the status block, the
//! receive buffer pool and the transmit buffer pool. A refused step gives
//! back every grant before it, in reverse, so the broker never holds a
//! partial setup. Grants are page aligned, which every ring base needs.

use nonos_libc::{mk_dma_map, DmaMapOut, MmioMapOut};

use crate::constants::layout::{RINGS_BYTES, RX_BUFFER_POOL_BYTES, TX_BUFFER_POOL_BYTES};

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

pub type Grants = (DmaMapOut, DmaMapOut, DmaMapOut);

pub fn map(device_id: u64, claim_epoch: u64, mmio: &MmioMapOut) -> Result<Grants, &'static str> {
    let rings = alloc(device_id, claim_epoch, RINGS_BYTES).ok_or_else(|| {
        rollback::after(device_id, mmio, &[]);
        "dma map refused (rings)"
    })?;
    let rx = alloc(device_id, claim_epoch, RX_BUFFER_POOL_BYTES).ok_or_else(|| {
        rollback::after(device_id, mmio, &[rings.grant_id]);
        "dma map refused (rx buffers)"
    })?;
    let tx = alloc(device_id, claim_epoch, TX_BUFFER_POOL_BYTES).ok_or_else(|| {
        rollback::after(device_id, mmio, &[rings.grant_id, rx.grant_id]);
        "dma map refused (tx buffers)"
    })?;
    Ok((rings, rx, tx))
}
