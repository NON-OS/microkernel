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

//! SERIAL's tail after the bus controller's key, 21 bytes, all zero where
//! it does not apply:
//!
//! - SPI, 4 bytes: data bits, clock phase, clock polarity, and mode bits
//!   (`THREE_WIRE`, `SELECT_HIGH`); the connection speed and chip-select
//!   line are SERIAL's speed and address fields;
//! - HID over SPI, 17 bytes: a mask of what firmware's _DSM gave
//!   (bit 0 input report header address, 1 input report body address,
//!   2 output report address, 3 read opcode, 4 write opcode, 5 flags),
//!   then those as u32, u32, u32, u8, u8, u16. Given to `driver.spi_hid0`
//!   alone, and only for a PNP0C51 device; anyone else reads zeros.

use super::state::Peripheral;
use nonos_aml::serial::Bus;

pub const THREE_WIRE: u8 = 1;
pub const SELECT_HIGH: u8 = 2;
pub const TAIL: usize = 21;

pub fn tail(p: &Peripheral, spi_hid_reader: bool) -> [u8; TAIL] {
    let mut t = [0u8; TAIL];
    if let Some(Bus::Spi { data_bits, phase, polarity, three_wire, select_active_high, .. }) =
        p.dev.connections.first().map(|c| &c.bus)
    {
        let wire = if *three_wire { THREE_WIRE } else { 0 };
        let mode = wire | if *select_active_high { SELECT_HIGH } else { 0 };
        t[..4].copy_from_slice(&[*data_bits, *phase, *polarity, mode]);
    }
    let Some(h) = p.hid_spi.filter(|_| spi_hid_reader) else { return t };
    let words = [h.input_header, h.input_body, h.output];
    let mut mask = 0u8;
    for (i, w) in words.iter().enumerate() {
        mask |= u8::from(w.is_some()) << i;
        t[5 + 4 * i..9 + 4 * i].copy_from_slice(&w.unwrap_or(0).to_le_bytes());
    }
    mask |= u8::from(h.read_opcode.is_some()) << 3 | u8::from(h.write_opcode.is_some()) << 4;
    mask |= u8::from(h.flags.is_some()) << 5;
    t[4] = mask;
    t[17] = h.read_opcode.unwrap_or(0);
    t[18] = h.write_opcode.unwrap_or(0);
    t[19..21].copy_from_slice(&h.flags.unwrap_or(0).to_le_bytes());
    t
}
