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

//! Reading the one cell whose circuit id is two bytes wide.

extern crate alloc;

use alloc::vec::Vec;

/// Read the first cell of a link.
///
pub fn parse_versions(bytes: &[u8]) -> Option<(Vec<u8>, usize)> {
    if bytes.len() < 5 {
        return None;
    }
    let length = u16::from_be_bytes([bytes[3], bytes[4]]) as usize;
    let total = 5 + length;
    Some((bytes.get(5..total)?.to_vec(), total))
}
