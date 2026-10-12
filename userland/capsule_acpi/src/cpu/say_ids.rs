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

//! Naming processors and register spaces on an `[ACPI] cpu` line.

use crate::sys::log::Line;
use nonos_aml::cpu::{FIXED_HARDWARE, PCC, SYSTEM_IO, SYSTEM_MEMORY};

pub fn space(s: u8) -> &'static [u8] {
    match s {
        FIXED_HARDWARE => b"FFH",
        SYSTEM_IO => b"I/O",
        SYSTEM_MEMORY => b"memory",
        PCC => b"PCC",
        _ => b"other",
    }
}

/// "0-3" when the UIDs run on, else the first and how many more.
pub fn uids(l: Line, uids: &[u64]) -> Line {
    let (first, last) = (uids[0], uids[uids.len() - 1]);
    let run = uids.windows(2).all(|w| w[1] == w[0].wrapping_add(1));
    match uids.len() {
        1 => l.dec(first),
        _ if run => l.dec(first).str(b"-").dec(last),
        n => l.dec(first).str(b" and ").dec(n as u64 - 1).str(b" alike"),
    }
}
