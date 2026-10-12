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
use crate::constants::VIRTIO_NET_HDR_LEN;
use crate::init::program_queues;
use crate::queue::{RxQueue, TxQueue};
use crate::regs::Regs;

/// Clear both rings, point the device at them, and build the queues over
/// them; `program_queues` refuses any ring size the grants cannot hold, and
/// resets the device when it does.
pub fn build(regs: Regs, dma: &DmaSet) -> Result<(RxQueue, TxQueue), &'static str> {
    dma.clear_rings();
    let (rx_ring, tx_ring) = (dma.rx_ring.device_addr, dma.tx_ring.device_addr);
    let (rx_count, tx_count) = program_queues(regs, rx_ring, tx_ring)?;
    Ok(dma.queues(rx_count, tx_count, VIRTIO_NET_HDR_LEN))
}
