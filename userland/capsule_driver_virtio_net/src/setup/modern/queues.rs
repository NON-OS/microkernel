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

//! The receive and transmit queues on the modern transport.
//!
//! The ring layout is the legacy one, unchanged: descriptors at 0, the
//! available ring at 4096 and the used ring at 8192 of one DMA region, and
//! ring positions taken modulo RING_SLOTS. Those offsets hold only for a
//! 256-entry ring, so the device is told exactly 256 (it may offer more;
//! QEMU offers 256) and one that cannot take 256 is refused rather than
//! driven with rings where it will not look.

use nonos_virtio::common::{setup_queue, NotifyArea, QueueSpec};
use nonos_virtio::{Mmio, VirtioError};

use super::super::dma_set::DmaSet;
use crate::constants::{
    Q_RX, Q_TX, RING_SLOTS, VIRTIO_NET_HDR_LEN_V1, VQ_AVAIL_OFFSET, VQ_DESC_OFFSET, VQ_REGION_SIZE,
    VQ_USED_OFFSET,
};
use crate::queue::{clear_region, RxQueue, TxQueue};

pub struct Queues {
    pub rx: RxQueue,
    pub tx: TxQueue,
    /// Doorbell offsets in the notify region, by queue index.
    pub doorbells: [usize; 2],
}

pub fn build(common: &Mmio, notify: NotifyArea, dma: &DmaSet) -> Result<Queues, &'static str> {
    unsafe {
        clear_region(dma.rx_queue.user_va, VQ_REGION_SIZE);
        clear_region(dma.tx_queue.user_va, VQ_REGION_SIZE);
    }
    let rx_spec = spec(Q_RX, dma.rx_queue.device_addr).ok_or("virtio-net: rx ring address")?;
    let tx_spec = spec(Q_TX, dma.tx_queue.device_addr).ok_or("virtio-net: tx ring address")?;
    let rx_ready = setup_queue(common, &rx_spec, notify).map_err(VirtioError::message)?;
    let tx_ready = setup_queue(common, &tx_spec, notify).map_err(VirtioError::message)?;
    let rx_count = core::cmp::min(rx_ready.size, RxQueue::queue_size());
    let tx_count = core::cmp::min(tx_ready.size, TxQueue::queue_size());
    let rx = RxQueue::new(
        dma.rx_queue.user_va,
        dma.rx_queue.device_addr,
        dma.rx_buffer.user_va,
        dma.rx_buffer.device_addr,
        rx_count,
    )
    .with_hdr_len(VIRTIO_NET_HDR_LEN_V1);
    let tx = TxQueue::new(
        dma.tx_queue.user_va,
        dma.tx_queue.device_addr,
        dma.tx_buffer.user_va,
        dma.tx_buffer.device_addr,
        tx_count,
    )
    .with_hdr_len(VIRTIO_NET_HDR_LEN_V1);
    Ok(Queues { rx, tx, doorbells: [rx_ready.notify_offset, tx_ready.notify_offset] })
}

fn spec(index: u16, base: u64) -> Option<QueueSpec> {
    Some(QueueSpec {
        index,
        size: RING_SLOTS,
        min_size: RING_SLOTS,
        desc: base.checked_add(VQ_DESC_OFFSET as u64)?,
        driver: base.checked_add(VQ_AVAIL_OFFSET as u64)?,
        device: base.checked_add(VQ_USED_OFFSET as u64)?,
        vector: None,
    })
}
