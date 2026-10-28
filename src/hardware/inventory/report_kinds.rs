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

//! What a line of the hardware report says: how the device is named and
//! what NONOS does with it.

/// The longest line `text` writes.
pub(super) const TEXT_MAX: usize = 120;

/// How a device is named on its line.
pub(super) enum Ids {
    /// A PCI function: vendor:device, class/subclass.
    Pci((u16, u16), (u8, u8)),
    /// An ACPI device with a PNP id, PNPxxxx.
    Pnp(u16),
    /// Any other bus, by its name and the record's two ids.
    Other(&'static [u8], (u16, u16)),
}

/// What NONOS does with one device, for its line.
pub(super) enum Verdict<'a> {
    /// The capsule that takes it.
    Driver(&'a str),
    /// Known, and no driver takes it, for this reason.
    Held(&'a str),
    /// A bridge or platform function nothing needs to drive.
    NotNeeded,
    /// No table names it.
    Unknown,
}
