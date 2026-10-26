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

//! Which writes switch an Intel PCH's shared ports from EHCI to this xHCI,
//! and which hand them back. Pure: xhci_proofs. Register offsets and
//! the order are from Linux's drivers/usb/host/pci-quirks.c (21a7f44fd076),
//! read for the facts alone; the kernel holds the device list and the masks
//! (quirk_bits/xhci_route.rs). Unverified on hardware.

pub const SUBSYSTEM: u32 = 0x2C;
pub const XUSB2PR: u32 = 0xD0;
pub const USB2PRM: u32 = 0xD4;
pub const USB3_PSSEN: u32 = 0xD8;
pub const USB3PRM: u32 = 0xDC;
/// The machine pci-quirks.c records as unable to switch: a Sony VAIO with
/// subsystem device 0x90a8.
const CANNOT_SWITCH: (u16, u16) = (0x104D, 0x90A8);

/// The ports firmware lets the OS route, as USB3PRM and USB2PRM read.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Masks {
    pub usb3: u16,
    pub usb2: u16,
}

/// Whether to switch at all: an Intel controller, not the machine that
/// cannot, with a port to move.
pub fn wants(vendor: u16, subsystem: (u16, u16), m: Masks) -> bool {
    vendor == 0x8086 && subsystem != CANNOT_SWITCH && (m.usb2 | m.usb3) != 0
}

/// The writes, `(register, mask, value)`, in order: SuperSpeed terminations
/// first, so a USB 3 device connects at SuperSpeed, then the USB 2.0 wires.
pub fn switch_steps(m: Masks) -> [(u32, u16, u16); 2] {
    [(USB3_PSSEN, m.usb3, m.usb3), (XUSB2PR, m.usb2, m.usb2)]
}

/// Back as firmware had them, the USB 2.0 wires first.
pub fn restore_steps(m: Masks) -> [(u32, u16, u16); 2] {
    [(XUSB2PR, m.usb2, 0), (USB3_PSSEN, m.usb3, 0)]
}

/// Whether a read-back shows the write took, in the bits it named.
pub fn took(read: u16, mask: u16, value: u16) -> bool {
    read & mask == value
}
