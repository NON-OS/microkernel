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

//! SDEV, the secure devices table: the devices firmware asks the OS to
//! keep secured (a device a secure OS or the firmware itself relies on),
//! by namespace path or PCIe path, each with whether it may be handed off.
//! After the header the entries follow at once, each with a u16 length at
//! 2 (`sdev_entry`). Recalled from ACPI 6.5, 5.2.27, unverified. An entry
//! of an unknown type is passed over; one that leaves the table ends the
//! walk as a structure error.

use super::le::u16_at;
use super::sdev_entry::entry;
pub use super::sdev_entry::{SdevEntry, SecureDevice};
use super::sdt::{table, TableError, HEADER};
use alloc::vec::Vec;

/// Entries kept.
pub const MAX_ENTRIES: usize = 64;

pub fn sdev(t: &[u8]) -> Result<Vec<SdevEntry>, TableError> {
    let t = table(t, b"SDEV")?;
    let (mut out, mut at) = (Vec::new(), HEADER);
    while at < t.len() && out.len() < MAX_ENTRIES {
        let len = usize::from(u16_at(t, at + 2).ok_or(TableError::Structure(at))?);
        let e = t.get(at..at + len).filter(|_| len >= 4).ok_or(TableError::Structure(at))?;
        out.extend(entry(e));
        at += len;
    }
    Ok(out)
}

/// A PCIe entry's bus, device and function: each hop but the last is a
/// bridge, whose secondary bus the next hop sits on. `secondary(bus, dev,
/// func)` reads it; None when a bridge does not answer.
pub fn resolve(
    start_bus: u8,
    path: &[(u8, u8)],
    secondary: impl Fn(u8, u8, u8) -> Option<u8>,
) -> Option<(u8, u8, u8)> {
    let (last, through) = path.split_last()?;
    let mut bus = start_bus;
    for &(dev, func) in through {
        bus = secondary(bus, dev, func)?;
    }
    Some((bus, last.0, last.1))
}
