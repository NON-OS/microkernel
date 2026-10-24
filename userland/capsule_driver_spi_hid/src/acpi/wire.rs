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

//! capsule_acpi's SERIAL (op 9), as F 65 gives it to this driver: a raw
//! request, the op byte then an index u32; the reply a status i32, then one
//! serial-bus record, little-endian throughout. Pure, for the proofs.

pub const OP_SERIAL: u8 = 9;
pub const KIND_SPI: u8 = 2;
/// The device's interrupt is a GpioInt.
pub const FLAG_GPIO_INT: u8 = 1 << 3;
/// The HID facts this driver cannot go without: the input header, input
/// body and output addresses, and the read and write opcodes.
pub const HID_NEEDED: u8 = 0b1_1111;
/// The record's fixed part, its four strings at most, the key, and the SPI
/// and HID blocks.
pub const REPLY_MAX: usize = 4 + 42 + 4 * 256 + 8 + 4 + 17;

pub fn request(index: u32) -> [u8; 5] {
    let i = index.to_le_bytes();
    [OP_SERIAL, i[0], i[1], i[2], i[3]]
}

/// One serial-bus device. Fields the firmware left unknown (all ones on the
/// wire) are `None`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Serial {
    pub count: u32,
    pub kind: u8,
    pub flags: u8,
    pub chip_select: u16,
    pub hz: u32,
    /// The GpioInt's first pin, else the first GSI.
    pub pin: u32,
    pub gpio_uid: Option<u64>,
    /// `1 << 63 | broker device id` of the bus controller.
    pub key: Option<u64>,
    pub data_bits: u8,
    pub phase: u8,
    pub polarity: u8,
    /// Bit 0 three-wire, bit 1 chip select active high.
    pub mode: u8,
    pub hid: Option<HidFacts>,
}

/// The firmware's HID-over-SPI facts, present only when all it needs are.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HidFacts {
    pub input_header: u32,
    pub input_body: u32,
    pub output: u32,
    pub read_opcode: u8,
    pub write_opcode: u8,
    pub flags: u16,
}
