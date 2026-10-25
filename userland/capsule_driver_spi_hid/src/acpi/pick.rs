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

//! Whether a serial-bus record is a device this driver can run over
//! driver.spi0, and what it is run with. spi0 moves 8-bit frames on four
//! wires, its one chip select driven low; a device asking anything else is
//! refused rather than driven wrong.

use nonos_spi_hid::Params;

use super::wire::{Serial, FLAG_GPIO_INT, KIND_SPI};

/// The _DSM flags' multi-SPI mode, bits 15:14; zero is single SPI.
const MULTI_SPI: u16 = 0b11 << 14;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Choice {
    /// spi0's mode: bit 1 clock polarity, bit 0 clock phase.
    pub mode: u8,
    pub hz: u32,
    /// Reads at the input header address, writes at the output address.
    pub params: Params,
    pub body_address: u32,
    /// The GpioInt's controller `_UID` and pin, as gpio0 names a line.
    pub line: Option<(u16, u16)>,
}

pub fn choose(s: &Serial, host_key: u64) -> Result<Choice, &'static str> {
    let hid = s.hid.ok_or("spi_hid: not a HID-over-SPI device")?;
    if s.kind != KIND_SPI || s.key != Some(host_key) {
        return Err("spi_hid: on another bus than spi0's");
    }
    if s.data_bits != 8 || s.mode != 0 || s.chip_select != 0 || hid.flags & MULTI_SPI != 0 {
        return Err("spi_hid: asks a wiring spi0 does not drive");
    }
    if s.phase > 1 || s.polarity > 1 || s.hz == 0 {
        return Err("spi_hid: a clock spi0 cannot give");
    }
    let params = Params {
        read_opcode: hid.read_opcode,
        write_opcode: hid.write_opcode,
        input_address: hid.input_header,
        output_address: hid.output,
    };
    let narrow = |v: u64| u16::try_from(v).ok();
    let line = match (s.flags & FLAG_GPIO_INT != 0, s.gpio_uid.and_then(narrow)) {
        (true, Some(uid)) => narrow(u64::from(s.pin)).map(|pin| (uid, pin)),
        _ => None,
    };
    let mode = s.polarity << 1 | s.phase;
    Ok(Choice { mode, hz: s.hz, params, body_address: hid.input_body, line })
}
