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

//! End-to-end broker handshake on the device discovery found: claim -> PCI
//! command (Memory Space, Bus Master off) -> MMIO -> RX ring DMA -> RX
//! buffer DMA -> TX ring DMA -> TX buffer DMA.
//! A failing step gives back what the earlier ones took. Returns a
//! `Driver` with all grants taken and ring states initialised; the
//! hardware bring-up step in `init` programs the device against those
//! rings.
//!
//! No interrupt line is bound. The driver polls and never sets IMS, and a
//! bound INTx line is masked until acked: holding one it never services
//! starved any other device sharing that line, and a failed bind (line
//! already held, or reserved) took down a NIC that needed no interrupt.

use nonos_libc::{mk_device_release, Deadline};

use crate::constants::queue::TX_HANG_MS;
use crate::constants::MAC_LEN;
use crate::discover::Found;
use crate::queue::{RxRing, TxRing};
use crate::regs::Regs;

use super::driver::Driver;
use super::{claim, dma, mmio, pci};

pub fn run(dev: Found) -> Result<Driver, &'static str> {
    let claim_epoch = claim::claim(dev.device_id)?;
    if let Err(e) = pci::command(dev.device_id, claim_epoch, false) {
        let _ = mk_device_release(dev.device_id);
        return Err(e);
    }
    let mmio_grant = mmio::map(dev, claim_epoch)?;
    let (rx_ring, rx_buf, tx_ring, tx_buf) =
        dma::map_rings_and_buffers(dev.device_id, claim_epoch, &mmio_grant)?;
    Ok(Driver {
        device_id: dev.device_id,
        claim_epoch,
        rx_ring_device_addr: rx_ring.device_addr,
        tx_ring_device_addr: tx_ring.device_addr,
        regs: Regs::new(mmio_grant.user_va),
        mac: [0u8; MAC_LEN],
        rx: RxRing::new(rx_ring.user_va, rx_buf.user_va, rx_buf.device_addr),
        tx: TxRing::new(tx_ring.user_va, tx_buf.user_va, tx_buf.device_addr),
        tx_watch: Deadline::after_ms(TX_HANG_MS),
    })
}
