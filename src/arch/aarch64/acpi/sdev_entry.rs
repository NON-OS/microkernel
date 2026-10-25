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

//! One SDEV entry (ACPI 6.5, 5.2.27; offsets recalled, unverified): type
//! u8 (0 an ACPI namespace device, 1 a PCIe endpoint), flags u8 (bit 0,
//! firmware allows the device to be handed off to the OS), length u16.
//! Type 0: the device identifier's offset and length u16 at 4 and 6, a
//! NUL-terminated namespace path. Type 1: the segment u16 at 4, the start
//! bus u16 at 6, then the path's offset and length u16 at 8 and 10, the
//! path being (device u8, function u8) hops from the start bus. Offsets
//! are from the entry's start and must land inside it.

use super::le::{u16_at, u8_at};
use alloc::vec::Vec;

/// Hops kept on one PCIe path.
const MAX_HOPS: usize = 8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SecureDevice {
    Acpi(Vec<u8>),
    Pci { segment: u16, start_bus: u8, path: Vec<(u8, u8)> },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SdevEntry {
    pub device: SecureDevice,
    /// Firmware lets the OS take the device over later.
    pub handoff: bool,
}

/// `e` is the entry, cut to its own length. None for an unknown type or an
/// offset that leaves it.
pub fn entry(e: &[u8]) -> Option<SdevEntry> {
    let field = |at: usize| u16_at(e, at).map(usize::from);
    let slice = |at: usize, len: usize| e.get(at..at.checked_add(len)?).filter(|_| at >= 12);
    let device = match u8_at(e, 0)? {
        0 => {
            let id = slice(field(4)?, field(6)?)?;
            SecureDevice::Acpi(id.split(|c| *c == 0).next().unwrap_or(&[]).to_vec())
        }
        1 => {
            let hops = slice(field(8)?, field(10)?)?;
            let path: Vec<_> = hops.chunks_exact(2).take(MAX_HOPS).map(|h| (h[0], h[1])).collect();
            let start_bus = u8::try_from(field(6)?).ok()?;
            (!path.is_empty()).then_some(())?;
            SecureDevice::Pci { segment: field(4)? as u16, start_bus, path }
        }
        _ => return None,
    };
    Some(SdevEntry { device, handoff: u8_at(e, 1)? & 1 != 0 })
}
