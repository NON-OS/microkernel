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

//! The driver record the setup sequence would hand over, and a port grant
//! the broker no longer honours.

use nonos_libc::{Deadline, Port};

use crate::constants::dma::TX_HANG_MS;
use crate::constants::MAC_LEN;
use crate::pio::Pio;
use crate::setup::Driver;

pub const DEVICE: u64 = 1;
pub const PIO_GRANT: u64 = 3;
pub const RX_DA: u64 = 0x0010_0000;
pub const TX_DA: u64 = 0x0020_0000;

/// A port grant the broker no longer honours.
pub struct Refused;

impl Port for Refused {
    fn read(&mut self, _offset: u16, _width: u8) -> Option<u32> {
        None
    }

    fn write(&mut self, _offset: u16, _width: u8, _value: u32) -> bool {
        false
    }
}

/// The record the setup sequence would hand over, before bring-up, with the
/// receive ring at `rx_user_va`.
pub fn driver_over(rx_user_va: u64) -> Driver {
    Driver {
        device_id: DEVICE,
        claim_epoch: 1,
        rx_user_va,
        rx_device_addr: RX_DA,
        tx_user_va: 0,
        tx_device_addr: TX_DA,
        rx_offset: 0,
        tx_cur: 0,
        tx_dirty: 0,
        pio: Pio::new(PIO_GRANT),
        mac: [0; MAC_LEN],
        tx_watch: Deadline::after_ms(TX_HANG_MS),
    }
}
