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

//! The broker handshake on the device discovery found: claim, both register
//! pages, then the three DMA grants. A failing step gives back what the
//! earlier ones took. No interrupt is bound: the driver polls, and intrCtrl
//! disables every interrupt in the device.

use crate::constants::layout::{RX_COMP_OFF, RX_RING1_OFF, RX_RING_OFF, TX_COMP_OFF, TX_RING_OFF};
use crate::constants::MAC_LEN;
use crate::discover::Found;
use crate::queue::{RxRing, TxRing};
use crate::regs::Regs;

use super::driver::Driver;
use super::{claim, dma, mmio, pci};

pub fn run(dev: Found) -> Result<Driver, &'static str> {
    let epoch = claim::claim(dev.device_id)?;
    pci::enable(dev.device_id, epoch)?;
    let (pt, vd) = mmio::map(dev, epoch)?;
    let g = dma::map_all(dev.device_id, epoch, &[pt.grant_id, vd.grant_id])?;
    let va = |off: usize| g.control.user_va + off as u64;
    let (rx_pool, tx_pool) = (&g.rx_pool, &g.tx_pool);
    Ok(Driver {
        device_id: dev.device_id,
        pt_grant: pt.grant_id,
        vd_grant: vd.grant_id,
        control_grant: g.control.grant_id,
        rx_pool_grant: rx_pool.grant_id,
        tx_pool_grant: tx_pool.grant_id,
        control_dev: g.control.device_addr,
        control_va: g.control.user_va,
        regs: Regs::new(pt.user_va, vd.user_va),
        mac: [0u8; MAC_LEN],
        rx: RxRing::new(
            va(RX_RING_OFF),
            va(RX_RING1_OFF),
            va(RX_COMP_OFF),
            rx_pool.user_va,
            rx_pool.device_addr,
        ),
        tx: TxRing::new(va(TX_RING_OFF), va(TX_COMP_OFF), tx_pool.user_va, tx_pool.device_addr),
    })
}
