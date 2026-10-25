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

//! The bytes a table's checksum may run over, pure (K11). Its declared
//! length is firmware's word: a length past MAX_TABLE_BYTES, or past the
//! end of what is mapped, would have the sum read memory that is not the
//! table, or fault. kernel_proofs::acpi_span.

use super::entry_count::MAX_TABLE_BYTES;

/// `length` bytes from `phys`, when that is at least `header` and at most
/// MAX_TABLE_BYTES, and ends at or below `mapped_end`.
pub const fn table_span(phys: u64, length: u32, header: usize, mapped_end: u64) -> Option<usize> {
    let len = length as u64;
    if len < header as u64 || len > MAX_TABLE_BYTES {
        return None;
    }
    match phys.checked_add(len) {
        Some(end) if end <= mapped_end => Some(length as usize),
        _ => None,
    }
}
