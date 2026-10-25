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

//! What the kernel copies for the capsule from an SMBIOS entry point,
//! decided before a byte of the table is read. Pure: the proof crates
//! mount it beside `smbios_entry`, and `smbios.rs` reads only what it names.

use super::smbios_entry::{parse_entry, TABLE_MAX};

/// Bytes `entry_len` needs to find the entry point's own length, which is
/// all of it the kernel copies: "_SM3_" puts the length at offset 6.
pub const LEN_AT: usize = 7;

/// The table's range [start, end), from the entry point's copied bytes:
/// none when they fail their checks, and never empty, never more than
/// `TABLE_MAX` bytes, never wrapping past the top of the address space.
pub fn table_span(entry: &[u8]) -> Option<(u64, u64)> {
    let e = parse_entry(entry)?;
    let len = u64::from(e.len);
    if len == 0 || len > u64::from(TABLE_MAX) {
        return None;
    }
    Some((e.table, e.table.checked_add(len)?))
}
