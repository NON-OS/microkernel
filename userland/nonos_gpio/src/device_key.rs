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

//! The key a device's pin is bound to: its bus controller, whose holder
//! alone hears the pin. Two forms share one u64, told apart by bit 63:
//!
//! - clear: a PCI function, segment << 16 | bus << 8 | devfn, for a bus
//!   controller that is one (an Intel LPSS I2C function);
//! - set: the broker device id of an ACPI platform device, for a bus
//!   controller that has no PCI function (an AMD AMDI0010).
//!
//! capsule_acpi's HOLDER takes either form and runs the kernel's claim
//! test for it. Pure, so the proofs can hold both forms.

pub const PLATFORM: u64 = 1 << 63;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    /// (segment, bus, device, function).
    Pci(u16, u8, u8, u8),
    Device(u64),
}

/// A root-bus PCI function from its `_ADR` (device in bits 16 to 20,
/// function in bits 0 to 2); segment 0 and bus 0, where these sit.
pub fn pci_key(adr: u64) -> u64 {
    ((adr >> 16) & 0x1F) << 3 | (adr & 7)
}

pub fn decode(key: u64) -> Key {
    if key & PLATFORM != 0 {
        return Key::Device(key & !PLATFORM);
    }
    let devfn = key as u8;
    Key::Pci((key >> 16) as u16, (key >> 8) as u8, devfn >> 3, devfn & 7)
}
