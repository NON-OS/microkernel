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

//! The one key a capsule names a bus controller by: `DEVICE | broker device
//! id`, for a controller that is a PCI function and for one that is an
//! ACPI platform device alike. A capsule never sees bus, device or function
//! numbers, so the PCI form (its `_ADR` down the path) is turned into the
//! device id here, inside capsule_acpi, and goes no further. Pure: the
//! proofs show both forms of one controller come out as the same key.

use alloc::vec::Vec;

pub const DEVICE: u64 = 1 << 63;

pub type Bdf = (u16, u8, u8, u8);

/// One device in the kernel's table, as capsule_acpi may look at it: its
/// PCI function when it is one, its namespace path when it has one.
pub struct Known {
    pub device_id: u64,
    pub bdf: Option<Bdf>,
    pub path: Vec<u8>,
}

/// How firmware names a serial bus's controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Controller<'a> {
    /// Its `_ADR` resolved to a function down the host bridge's path.
    Pci(Bdf),
    /// Its namespace path, for a controller with no `_ADR`.
    Path(&'a [u8]),
}

/// None when no device in the table is that controller, or its id would
/// not fit beside the bit.
pub fn key(c: Controller<'_>, known: &[Known]) -> Option<u64> {
    let found = known.iter().find(|k| match c {
        Controller::Pci(f) => k.bdf == Some(f),
        Controller::Path(p) => !p.is_empty() && k.path == p,
    })?;
    (found.device_id & DEVICE == 0).then_some(DEVICE | found.device_id)
}

/// The broker device id a key names; None for anything but this form.
pub fn device_of(key: u64) -> Option<u64> {
    (key & DEVICE != 0).then_some(key & !DEVICE)
}
