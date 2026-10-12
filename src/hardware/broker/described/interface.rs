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

use crate::hardware::broker::class::ids;

/// The register interface a described device presents. The value is its
/// record's `device` field, part of the broker ABI.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interface {
    /// SD Host Controller Specification registers.
    Sdhci = 1,
    /// eXtensible Host Controller Interface registers.
    Xhci = 2,
    /// Serial ATA AHCI registers.
    Ahci = 3,
}

impl Interface {
    /// Broker class, and the class code, subclass and programming interface
    /// the same registers carry on PCI (PCI Code and ID Assignment): what a
    /// capsule written for the PCI function already matches on.
    pub(super) const fn class(self) -> (u32, u8, u8, u8) {
        match self {
            Interface::Sdhci => (ids::BLOCK, 0x08, 0x05, 0x01),
            Interface::Xhci => (ids::USB_HOST_XHCI, 0x0c, 0x03, 0x30),
            Interface::Ahci => (ids::BLOCK, 0x01, 0x06, 0x01),
        }
    }
}
