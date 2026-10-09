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

use nonos_libc::MK_DMA_MAP_DMA32;

use crate::chip::{detect, has_gmii};
use crate::constants::MAC_LEN;
use crate::discover::Found;
use crate::queue::{RxRing, TxRing};
use crate::regs::Regs;

use super::driver::Driver;
use super::{claim, dma, mmio, pci, rollback};

/// Claim the card discovery found and take its grants. A failing step gives
/// back what the earlier ones took.
pub fn run(dev: Found) -> Result<Driver, &'static str> {
    let claim_epoch = claim::claim(dev.device_id)?;
    pci::enable_bus_master(dev, claim_epoch)?;
    let mmio = mmio::map(dev, claim_epoch)?;
    // Before any DMA is mapped: a chip that is refused has only the BAR and
    // the claim to give back.
    let gmii = has_gmii(dev.pci_device);
    let chip = detect(&Regs::new(mmio.user_va), mmio.length, gmii).map_err(|e| {
        rollback::after(dev.device_id, &mmio, &[]);
        e.as_str()
    })?;
    let flags = if chip.ver.dma32_only() { MK_DMA_MAP_DMA32 } else { 0 };
    let (rx_ring, rx_buf, tx_ring, tx_buf) =
        dma::map_all(dev.device_id, claim_epoch, &mmio, flags)?;
    Ok(Driver {
        device_id: dev.device_id,
        chip,
        mmio_grant: mmio.grant_id,
        rx_ring_grant: rx_ring.grant_id,
        rx_buffer_grant: rx_buf.grant_id,
        tx_ring_grant: tx_ring.grant_id,
        tx_buffer_grant: tx_buf.grant_id,
        regs: Regs::new(mmio.user_va),
        mac: [0u8; MAC_LEN],
        link: None,
        rx: RxRing::new(rx_ring.user_va, rx_buf.user_va, rx_ring.device_addr, rx_buf.device_addr),
        tx: TxRing::new(tx_ring.user_va, tx_buf.user_va, tx_ring.device_addr, tx_buf.device_addr),
    })
}
