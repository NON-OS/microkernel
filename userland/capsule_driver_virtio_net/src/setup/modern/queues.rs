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

use nonos_virtio::common::{setup_queue, NotifyArea};
use nonos_virtio::{Mmio, VirtioError};

use super::super::dma_set::DmaSet;
use super::spec::spec;
use crate::constants::{Q_RX, Q_TX, VIRTIO_NET_HDR_LEN_V1};
use crate::queue::{RxQueue, TxQueue};

pub struct Queues {
    pub rx: RxQueue,
    pub tx: TxQueue,
    /// Doorbell offsets in the notify region, by queue index.
    pub doorbells: [usize; 2],
}

pub fn build(common: &Mmio, notify: NotifyArea, dma: &DmaSet) -> Result<Queues, &'static str> {
    dma.clear_rings();
    let rx_spec = spec(Q_RX, dma.rx_ring.device_addr).ok_or("virtio-net: rx ring address")?;
    let tx_spec = spec(Q_TX, dma.tx_ring.device_addr).ok_or("virtio-net: tx ring address")?;
    let rx_ready = setup_queue(common, &rx_spec, notify).map_err(VirtioError::message)?;
    let tx_ready = setup_queue(common, &tx_spec, notify).map_err(VirtioError::message)?;
    let (rx, tx) = dma.queues(rx_ready.size, tx_ready.size, VIRTIO_NET_HDR_LEN_V1);
    Ok(Queues { rx, tx, doorbells: [rx_ready.notify_offset, tx_ready.notify_offset] })
}
