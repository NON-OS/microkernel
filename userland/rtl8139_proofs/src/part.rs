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

//! The part behind the port grant, and the driver record over it, shared by
//! the bring-up and receive tests.

use nonos_libc::{attach, Attached, Port};

use crate::constants::regs::{CMD_RESET, REG_CMD};
use crate::constants::MAC_LEN;
use crate::pio::Pio;
use crate::setup::Driver;

pub const DEVICE: u64 = 1;
pub const PIO_GRANT: u64 = 3;
pub const RX_GRANT: u64 = 4;
pub const TX_GRANT: u64 = 5;

/// The register file of the part: what is written reads back, and a reset
/// request clears itself unless the part is stuck in reset.
pub struct Part {
    pub regs: [u8; 256],
    pub stuck_in_reset: bool,
}

impl Port for Part {
    fn read(&mut self, offset: u16, width: u8) -> Option<u32> {
        let at = offset as usize;
        let mut v = 0u32;
        for i in 0..width as usize {
            v |= (*self.regs.get(at + i)? as u32) << (8 * i);
        }
        Some(v)
    }

    fn write(&mut self, offset: u16, width: u8, value: u32) -> bool {
        let at = offset as usize;
        for i in 0..width as usize {
            match self.regs.get_mut(at + i) {
                Some(b) => *b = (value >> (8 * i)) as u8,
                None => return false,
            }
        }
        if offset == REG_CMD && !self.stuck_in_reset {
            self.regs[REG_CMD as usize] &= !CMD_RESET;
        }
        true
    }
}

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

pub fn part(stuck_in_reset: bool) -> Attached {
    attach(Box::new(Part { regs: [0; 256], stuck_in_reset }))
}

/// The record the setup sequence would hand over, before bring-up, with the
/// receive ring at `rx_user_va`.
pub fn driver_over(rx_user_va: u64) -> Driver {
    Driver {
        device_id: DEVICE,
        pio_grant: PIO_GRANT,
        rx_grant: RX_GRANT,
        tx_grant: TX_GRANT,
        rx_user_va,
        rx_device_addr: 0x0010_0000,
        tx_user_va: 0,
        tx_device_addr: 0x0020_0000,
        rx_offset: 0,
        tx_cur: 0,
        tx_dirty: 0,
        pio: Pio::new(PIO_GRANT),
        mac: [0; MAC_LEN],
    }
}
