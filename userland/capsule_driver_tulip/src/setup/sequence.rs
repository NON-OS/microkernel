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

//! The broker handshake on the device discovery found: claim, the command
//! bits, MMIO, then the three DMA grants. A failing step gives back what the
//! earlier ones took. No interrupt is bound: the driver polls with every
//! interrupt masked in CSR7 and the pin silenced.

use crate::constants::layout::{RX_RING_OFF, TX_RING_OFF};
use crate::constants::MAC_LEN;
use crate::discover::Found;
use crate::queue::{RxRing, TxRing};
use crate::regs::Regs;

use super::driver::Driver;
use super::{claim, dma, mmio, pci};

pub fn run(dev: Found) -> Result<Driver, &'static str> {
    let epoch = claim::claim(dev.device_id)?;
    pci::enable(dev.device_id, epoch)?;
    let window = mmio::map(dev, epoch)?;
    let g = dma::map_all(dev.device_id, epoch, &window)?;
    let ring = |off: usize| (g.control.user_va + off as u64, g.control.device_addr + off as u64);
    let regs = Regs::new(window.user_va);
    Ok(Driver {
        device_id: dev.device_id,
        device: dev.device,
        mmio_grant: window.grant_id,
        control_grant: g.control.grant_id,
        rx_pool_grant: g.rx_pool.grant_id,
        tx_pool_grant: g.tx_pool.grant_id,
        regs,
        mac: [0u8; MAC_LEN],
        rx: RxRing::new(ring(RX_RING_OFF), (g.rx_pool.user_va, g.rx_pool.device_addr), regs),
        tx: TxRing::new(ring(TX_RING_OFF), (g.tx_pool.user_va, g.tx_pool.device_addr)),
    })
}
