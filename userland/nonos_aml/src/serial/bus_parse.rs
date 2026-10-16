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

//! Serial Bus Connection descriptors read out of a template. Each type
//! has a minimum of type-specific data; a descriptor shorter than its
//! header, than the type's data, or than the data length it claims is
//! skipped, as is a type NONOS does not know (CSI-2 among them), and an SPI
//! one with a reserved value (`spi`).

use super::bus::{Bus, Connection};
use super::le::{u16_at, u32_at};
use super::spi::{spi, SPI_DATA};
use crate::platform::items;
use alloc::vec::Vec;

const SERIAL_BUS: u8 = 0x0E;
const I2C: u8 = 1;
const SPI: u8 = 2;
const UART: u8 = 3;

/// `b` is the descriptor body: revision, source index, type, general
/// flags, type flags (2), type revision, type data length (2), then data.
fn connection(b: &[u8]) -> Option<Connection> {
    let header = 9;
    let (kind, general, flags) = (*b.get(2)?, *b.get(3)?, u16_at(b.get(..6)?, 4));
    let len = u16_at(b.get(..9)?, 7) as usize;
    let d = b.get(header..header + len)?;
    let bus = match kind {
        I2C if len >= 6 => {
            Bus::I2c { address: u16_at(d, 4), speed_hz: u32_at(d, 0), ten_bit: flags & 1 != 0 }
        }
        SPI if len >= SPI_DATA => spi(d, flags)?,
        UART if len >= 10 => Bus::Uart {
            baud: u32_at(d, 0),
            rx_fifo: u16_at(d, 4),
            tx_fifo: u16_at(d, 6),
            parity: d[8],
            flags,
            lines: d[9],
        },
        _ => return None,
    };
    let source = b[header + len..].iter().copied().take_while(|c| *c != 0).collect();
    Some(Connection { bus, source, consumer: general & 2 != 0, shared: general & 4 != 0 })
}

pub fn serial_buses(t: &[u8]) -> Vec<Connection> {
    items(t)
        .iter()
        .filter(|i| i.large && i.kind == SERIAL_BUS)
        .filter_map(|i| connection(i.body))
        .collect()
}
