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

//! The broker handshake on the device discovery found: claim -> PCI command
//! (Memory Space, Bus Master off) -> MMIO -> RX ring DMA -> RX buffer DMA ->
//! TX ring DMA -> TX buffer DMA.
//! A failing step gives back what the earlier ones took. The hardware
//! bring-up in `init` then programs the part against these rings.
//!
//! No interrupt line is bound. The driver polls and masks every cause, and a
//! bound INTx line it never services would starve whatever shares it.

use nonos_libc::{mk_device_release, Deadline};

use crate::constants::timeouts::TX_HANG_MS;
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
        pci_device: dev.pci_device,
        rx_ring_device_addr: rx_ring.device_addr,
        tx_ring_device_addr: tx_ring.device_addr,
        regs: Regs::new(mmio_grant.user_va),
        mac: [0u8; MAC_LEN],
        rx: RxRing::new(rx_ring.user_va, rx_buf.user_va, rx_buf.device_addr),
        tx: TxRing::new(tx_ring.user_va, tx_buf.user_va, tx_buf.device_addr),
        link: None,
        tx_watch: Deadline::after_ms(TX_HANG_MS),
    })
}
