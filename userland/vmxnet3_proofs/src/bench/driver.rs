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
use crate::constants::layout::{RX_COMP_OFF, RX_RING1_OFF, RX_RING_OFF, TX_COMP_OFF, TX_RING_OFF};
use crate::queue::{RxRing, TxRing};
use crate::regs::Regs;
use crate::setup::Driver;

impl Bench {
    pub fn driver(&mut self) -> Driver {
        let control = self.control.as_mut_ptr() as u64;
        let va = |off: usize| control + off as u64;
        let (rx_pool, tx_pool) = (self.rx.as_mut_ptr() as u64, self.tx.as_mut_ptr() as u64);
        Driver {
            device_id: GRANTS[0],
            pt_grant: GRANTS[1],
            vd_grant: GRANTS[2],
            control_grant: GRANTS[3],
            rx_pool_grant: GRANTS[4],
            tx_pool_grant: GRANTS[5],
            control_dev: CONTROL_DEV,
            control_va: control,
            regs: Regs::new(0, 1),
            mac: [0; 6],
            rx: RxRing::new(va(RX_RING_OFF), va(RX_RING1_OFF), va(RX_COMP_OFF), rx_pool, RX_DEV),
            tx: TxRing::new(va(TX_RING_OFF), va(TX_COMP_OFF), tx_pool, TX_DEV),
        }
    }
}
