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

//! The receive and transmit queues on the legacy transport.

use super::dma_set::DmaSet;
use crate::constants::{Q_RX, Q_TX, VIRTIO_NET_HDR_LEN};
use crate::init::program_queue;
use crate::queue::{RxQueue, TxQueue};
use crate::regs::Regs;

/// Clear both rings, point the device at them, and build the queues over
/// them; `program_queue` refuses any ring size the grants cannot hold.
pub fn build(regs: Regs, dma: &DmaSet) -> Result<(RxQueue, TxQueue), &'static str> {
    dma.clear_rings();
    let rx_count = program_queue(regs, Q_RX, dma.rx_ring.device_addr, RxQueue::queue_size())?;
    let tx_count = program_queue(regs, Q_TX, dma.tx_ring.device_addr, TxQueue::queue_size())?;
    Ok(dma.queues(rx_count, tx_count, VIRTIO_NET_HDR_LEN))
}
