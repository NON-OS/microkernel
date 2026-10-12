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

//! A quirk: the machines it applies to (every listed SMBIOS field matched
//! exactly or by prefix), the device it applies to, the value the driver
//! reads, and why it exists.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Field {
    Vendor,
    Product,
    Version,
    BoardVendor,
    Board,
    BiosVendor,
    BiosVersion,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Match {
    Exact(&'static str),
    Prefix(&'static str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DmiMatch {
    pub field: Field,
    pub is: Match,
}

/// The device a quirk names, as its holder knows it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceMatch {
    Pci { vendor: u16, device: u16 },
    Acpi { hid: &'static str },
}

/// The device a driver holds, as the lookup is given it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Device<'a> {
    Pci { vendor: u16, device: u16 },
    Acpi { hid: &'a [u8] },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Quirk {
    /// Unique in the table; what the driver asks for by name.
    pub name: &'static str,
    /// Where the fault is documented: an erratum, a vendor note, a bug
    /// report, a measurement. Never empty.
    pub reason: &'static str,
    pub dmi: &'static [DmiMatch],
    pub device: DeviceMatch,
    pub value: u32,
}
