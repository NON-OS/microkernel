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

//! The four DMA grants: each queue's ring and its buffers.

use nonos_libc::MK_DMA_MAP_COHERENT;

use super::dma::{map, Grant};
use crate::constants::{
    RX_BUFFER_LEN, RX_DESC_COUNT, TX_BUFFER_LEN, TX_DESC_COUNT, VQ_REGION_SIZE,
};
use crate::queue::{zero, RxQueue, TxQueue};

const RX_BUFS: usize = RX_BUFFER_LEN as usize * RX_DESC_COUNT as usize;
const TX_BUFS: usize = TX_BUFFER_LEN as usize * TX_DESC_COUNT as usize;

pub struct DmaSet {
    pub rx_ring: Grant,
    pub rx_bufs: Grant,
    pub tx_ring: Grant,
    pub tx_bufs: Grant,
}

pub fn map_all(device_id: u64, claim_epoch: u64) -> Result<DmaSet, &'static str> {
    // The rings are read and written by both sides while they run: mapped
    // uncached, so neither needs a cache flush (virtio 1.2, 2.7.13).
    let ring = MK_DMA_MAP_COHERENT;
    Ok(DmaSet {
        rx_ring: map(device_id, claim_epoch, VQ_REGION_SIZE, ring, "dma map failed (rx queue)")?,
        rx_bufs: map(device_id, claim_epoch, RX_BUFS, 0, "dma map failed (rx buffers)")?,
        tx_ring: map(device_id, claim_epoch, VQ_REGION_SIZE, ring, "dma map failed (tx queue)")?,
        tx_bufs: map(device_id, claim_epoch, TX_BUFS, 0, "dma map failed (tx buffer)")?,
    })
}

impl DmaSet {
    /// Clear both rings, before the device is told where they are.
    pub fn clear_rings(&self) {
        zero(self.rx_ring.view, 0, VQ_REGION_SIZE);
        zero(self.tx_ring.view, 0, VQ_REGION_SIZE);
    }

    /// Both queues over the grants, `hdr_len` bytes of header per frame.
    pub fn queues(&self, rx_count: u16, tx_count: u16, hdr_len: usize) -> (RxQueue, TxQueue) {
        let (r, t) = (self.rx_bufs, self.tx_bufs);
        let rx = RxQueue::new(self.rx_ring.view, r.view, r.device_addr, rx_count);
        let tx = TxQueue::new(self.tx_ring.view, t.view, t.device_addr, tx_count);
        (rx.with_hdr_len(hdr_len), tx.with_hdr_len(hdr_len))
    }
}
