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

use nonos_libc::{mk_device_release, Deadline};

use crate::chip::{detect, has_gmii};
use crate::constants::queue::TX_HANG_MS;
use crate::constants::MAC_LEN;
use crate::discover::Found;
use crate::queue::{RxRing, TxRing};
use crate::regs::Regs;

use super::driver::Driver;
use super::{claim, dma, mmio, pci, rollback};

/// Claim the card discovery found, with Bus Master off until the bring-up
/// has reset the part, and take its grants. A failing step gives back what
/// the earlier ones took.
pub fn run(dev: Found) -> Result<Driver, &'static str> {
    let claim_epoch = claim::claim(dev.device_id)?;
    if let Err(e) = pci::command(dev.device_id, claim_epoch, false) {
        let _ = mk_device_release(dev.device_id);
        return Err(e);
    }
    let mmio = mmio::map(dev, claim_epoch)?;
    // Before any DMA is mapped: a chip that is refused has only the BAR and
    // the claim to give back.
    let gmii = has_gmii(dev.pci_device);
    let chip = detect(&Regs::new(mmio.user_va), mmio.length, gmii).map_err(|e| {
        rollback::unclaim(dev.device_id, claim_epoch);
        e.as_str()
    })?;
    let (rx_ring, rx_buf, tx_ring, tx_buf) =
        dma::map_all(dev.device_id, claim_epoch, &mmio, chip.ver)?;
    Ok(Driver {
        device_id: dev.device_id,
        claim_epoch,
        chip,
        regs: Regs::new(mmio.user_va),
        mac: [0u8; MAC_LEN],
        link: None,
        rx: RxRing::new(rx_ring.user_va, rx_buf.user_va, rx_ring.device_addr, rx_buf.device_addr),
        tx: TxRing::new(tx_ring.user_va, tx_buf.user_va, tx_ring.device_addr, tx_buf.device_addr),
        tx_watch: Deadline::after_ms(TX_HANG_MS),
    })
}
