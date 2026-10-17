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

//! I2C host controllers the firmware declares as PCI functions (Intel LPSS
//! from Skylake and Apollo Lake on): a Device with an `_ADR`, which is
//! device << 16 | function on the bus of the host bridge that encloses it.
//! A touchpad's ResourceSource names one by its NameSeg, and the broker maps
//! it to the PCI function it enumerated (K10g). Pure over the bytes.

use alloc::vec::Vec;

use super::names::named_integer;
use super::walk::devices;

/// A Device with a constant `_ADR`, by its NameSeg.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I2cPciHost {
    pub name: [u8; 4],
    pub adr: u32,
}

/// Every Device in `aml` with a constant `_ADR`. No `_HID` is asked for:
/// LPSS functions carry none, and only a name a touchpad names is used.
pub fn parse_pci_controllers(aml: &[u8]) -> Vec<I2cPciHost> {
    devices(aml)
        .into_iter()
        .filter_map(|node| {
            let adr = u32::try_from(named_integer(node.body, b"_ADR")?).ok()?;
            Some(I2cPciHost { name: node.name, adr })
        })
        .collect()
}
