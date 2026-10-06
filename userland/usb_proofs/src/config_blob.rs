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

//! Configuration descriptors as a device would send them, built record by
//! record so a test can break exactly one field.

use crate::descriptors::binding::HidBinding;
use crate::descriptors::types::{HidKind, DT_CONFIGURATION, DT_ENDPOINT, DT_INTERFACE};

/// The HID class descriptor that sits between an interface and its
/// endpoints; the walk must step over it.
const DT_HID: u8 = 0x21;
pub const HID: u8 = 0x03;
pub const INTERRUPT: u8 = 0x03;

pub struct Blob {
    bytes: Vec<u8>,
}

/// A 9-byte configuration header; `build` fills in wTotalLength.
pub fn config() -> Blob {
    Blob { bytes: vec![9, DT_CONFIGURATION, 0, 0, 1, 1, 0, 0xa0, 50] }
}

impl Blob {
    pub fn iface(self, number: u8, class: u8, subclass: u8, protocol: u8) -> Self {
        self.raw(&[9, DT_INTERFACE, number, 0, 1, class, subclass, protocol, 0])
    }

    pub fn hid(self) -> Self {
        self.raw(&[9, DT_HID, 0x11, 0x01, 0, 1, 0x22, 63, 0])
    }

    pub fn ep(self, address: u8, attributes: u8, max_packet: u16, interval: u8) -> Self {
        let [lo, hi] = max_packet.to_le_bytes();
        self.raw(&[7, DT_ENDPOINT, address, attributes, lo, hi, interval])
    }

    pub fn raw(mut self, bytes: &[u8]) -> Self {
        self.bytes.extend_from_slice(bytes);
        self
    }

    pub fn size(&self) -> usize {
        self.bytes.len()
    }

    /// The blob with wTotalLength naming exactly its own bytes.
    pub fn build(self) -> Vec<u8> {
        let total = self.bytes.len() as u16;
        self.build_with_total(total)
    }

    /// The blob with wTotalLength set to `total`, whatever the length.
    pub fn build_with_total(mut self, total: u16) -> Vec<u8> {
        self.bytes[2..4].copy_from_slice(&total.to_le_bytes());
        self.bytes
    }
}

/// A boot keyboard: HID interface, HID descriptor, interrupt IN endpoint 1.
pub fn keyboard(number: u8) -> Blob {
    config().iface(number, HID, 1, 1).hid().ep(0x81, INTERRUPT, 8, 10)
}

/// What a test compares a binding by.
pub fn summary(b: &HidBinding) -> (HidKind, u8, u8, u16) {
    (b.kind, b.interface_number, b.endpoint_address, b.max_packet_size)
}

pub fn summaries(bindings: &[HidBinding]) -> Vec<(HidKind, u8, u8, u16)> {
    bindings.iter().map(summary).collect()
}
