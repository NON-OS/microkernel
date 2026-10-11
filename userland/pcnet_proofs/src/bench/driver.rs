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

//! A driver over the bench's memory, as `setup::run` would leave it.

use super::memory::{Bench, CONTROL_DEV, GRANTS, RX_DEV, TX_DEV};
use crate::constants::layout::{RX_RING_OFF, TX_RING_OFF};
use crate::queue::{RxRing, TxRing};
use crate::regs::Regs;
use crate::setup::Driver;

impl Bench {
    pub fn driver(&mut self) -> Driver {
        let control = self.control.as_mut_ptr() as u64;
        let ring = |off: usize| (control + off as u64, (CONTROL_DEV as usize + off) as u64);
        let ((rx_va, rx_dev), (tx_va, tx_dev)) = (ring(RX_RING_OFF), ring(TX_RING_OFF));
        Driver {
            device_id: GRANTS[0],
            mmio_grant: GRANTS[1],
            control_grant: GRANTS[2],
            rx_pool_grant: GRANTS[3],
            tx_pool_grant: GRANTS[4],
            control_device_addr: CONTROL_DEV as u64,
            control_va: control,
            regs: Regs::new(0),
            mac: [0; 6],
            part: 0,
            rx: RxRing::new(rx_va, rx_dev, self.rx.as_mut_ptr() as u64, RX_DEV as u64),
            tx: TxRing::new(tx_va, tx_dev, self.tx.as_mut_ptr() as u64, TX_DEV as u64),
        }
    }
}
