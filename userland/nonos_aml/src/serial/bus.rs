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

//! What a Serial Bus Connection descriptor says (ACPI 6.5, 6.4.3.8.2):
//! which controller (its ResourceSource), and how to reach the device on
//! that bus.

use alloc::vec::Vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Bus {
    I2c {
        /// 7 bits, or 10 with `ten_bit`.
        address: u16,
        speed_hz: u32,
        ten_bit: bool,
    },
    Spi {
        speed_hz: u32,
        data_bits: u8,
        /// Clock phase: 0 samples on the first edge, 1 on the second.
        phase: u8,
        /// Clock polarity: 0 idles low, 1 idles high.
        polarity: u8,
        chip_select: u16,
        three_wire: bool,
        select_active_high: bool,
    },
    Uart {
        baud: u32,
        rx_fifo: u16,
        tx_fifo: u16,
        /// 0 none, 1 even, 2 odd, 3 mark, 4 space.
        parity: u8,
        /// The type-specific flags as given: flow control in bits 1:0,
        /// stop bits in 3:2, data bits in 6:4, big-endian in bit 7.
        flags: u16,
        lines: u8,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Connection {
    pub bus: Bus,
    /// The controller's path as text, up to its NUL.
    pub source: Vec<u8>,
    /// The device is the controller's consumer (it does not drive the bus).
    pub consumer: bool,
    pub shared: bool,
}
