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

//! The rings in host memory and a driver built over them.

use nonos_devmodel::FakeBar;
use nonos_libc::Deadline;

use crate::constants::queue::TX_BUFFER_POOL_BYTES;
use crate::constants::queue::{RX_BUFFER_POOL_BYTES, RX_DESC_COUNT, TX_DESC_COUNT, TX_HANG_MS};
use crate::constants::MAC_LEN;
use crate::queue::layout::{RxDesc, TxDesc};
use crate::queue::{RxRing, TxRing};
use crate::regs::Regs;
use crate::setup::Driver;

pub const RX_RING_PHYS: u64 = 0x1_0000_1000;
pub const TX_RING_PHYS: u64 = 0x1_0000_2000;
pub const RX_BUF_PHYS: u64 = 0x1_0010_0000;
pub const TX_BUF_PHYS: u64 = 0x1_0020_0000;

pub struct Memory {
    pub rx: Box<[RxDesc; RX_DESC_COUNT]>,
    pub tx: Box<[TxDesc; TX_DESC_COUNT]>,
    pub rx_buf: Vec<u8>,
    pub tx_buf: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            rx: Box::new([RxDesc::default(); RX_DESC_COUNT]),
            tx: Box::new([TxDesc::default(); TX_DESC_COUNT]),
            rx_buf: vec![0; RX_BUFFER_POOL_BYTES],
            tx_buf: vec![0; TX_BUFFER_POOL_BYTES],
        }
    }

    pub fn rx_ring(&mut self) -> RxRing {
        RxRing::new(self.rx.as_mut_ptr() as u64, self.rx_buf.as_mut_ptr() as u64, RX_BUF_PHYS)
    }

    pub fn tx_ring(&mut self) -> TxRing {
        TxRing::new(self.tx.as_mut_ptr() as u64, self.tx_buf.as_mut_ptr() as u64, TX_BUF_PHYS)
    }

    /// The live state as the setup sequence would leave it, before bring-up.
    pub fn driver(&mut self, bar: &FakeBar) -> Driver {
        Driver {
            device_id: 1,
            claim_epoch: 1,
            rx_ring_device_addr: RX_RING_PHYS,
            tx_ring_device_addr: TX_RING_PHYS,
            regs: Regs::new(bar.base()),
            mac: [0u8; MAC_LEN],
            rx: self.rx_ring(),
            tx: self.tx_ring(),
            tx_watch: Deadline::after_ms(TX_HANG_MS),
        }
    }
}
