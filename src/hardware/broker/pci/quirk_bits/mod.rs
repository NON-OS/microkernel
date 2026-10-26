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

//! The few vendor bits a driver of a claimed device may flip beyond Command
//! and MSI-X, each named for its register in the vendor's datasheet, and
//! nothing beside them. Pure, so kernel_proofs holds every rule. The names
//! and bit positions are recalled from Intel's PCH datasheets, AMD's and the
//! PCI Express Base Specification, unverified until the owner's row (L37).
//!
//! - Intel HD Audio (class 04, subclass 03, or 01 for a DSP-mode part):
//!   TCSEL at 0x44 bits 0-2, the traffic class the controller's requests
//!   carry; CGCTL at 0x48 bit 6 (MISCBDCGE), a clock gate held off around
//!   the controller reset on the newer PCHs; DEVC at 0x78 bit 11, no-snoop.
//! - AMD/ATI HD Audio: the snoop control bits at 0x42 bits 0-2.
//! - A network controller (class 02): PCI Express Device Control 2's
//!   completion timeout value (bits 0-3) and disable (bit 4), at the PCIe
//!   capability + 0x28, for parts whose reads outlast the default range.
//! - The Intel SMBus host (class 0C, subclass 05): HOSTC at 0x40, host
//!   enable (bit 0) and I2C mode (bit 2).
//! - A UHCI controller (class 0C, subclass 03, interface 00), any vendor:
//!   LEGSUP at 0xC0, its firmware trap and SMI enables (7:0), their status
//!   (11:8, 15, each cleared by a one) and PIRQD routing (13), so a polled
//!   driver turns firmware's legacy keyboard emulation off. Bits 12 and 14
//!   stay as read (Intel PIIX4 and ICH datasheets, as recalled, unverified).
//! - An EHCI controller (class 0C, subclass 03, interface 20), any vendor:
//!   in its USB legacy support capability, wherever its id byte (01h) is,
//!   the OS-owned semaphore set, and the SMI enables and their status, so
//!   the driver takes the controller from firmware (EHCI 1.0 section 5.1,
//!   as recalled, unverified). Never inside a capability on the standard
//!   list: the Power Management capability shares the id 01h (listed.rs).
//!
//! - The Intel 7, 8 and 9 series PCH xHCI (8086, 0C/03/30, the ids
//!   xhci_route.rs cites): XUSB2PR and USB3_PSSEN, within the masks firmware
//!   sets in USB2PRM and USB3PRM, while no EHCI on its bus is claimed. That
//!   moves ports between the two host controllers on one PCH and gives no
//!   device new memory reach: a port routed to the xHCI is reached only
//!   through the claimed xHCI's own rings and IOMMU domain.
//!
//! Anything else stays refused: these registers hold no address and no BAR,
//! and change nothing about what the device may reach.

mod ident;
mod legsup;
mod listed;
mod span;
mod writable;
mod xhci_route;

pub(crate) use ident::{Ident, Route};
pub(crate) use legsup::legsup_before;
pub(crate) use writable::{only, writable};
pub(crate) use xhci_route::blocks;
