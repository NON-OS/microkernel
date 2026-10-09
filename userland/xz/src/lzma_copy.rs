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

//! Copying a match out of the dictionary, which is the output since the
//! last reset.

use alloc::vec::Vec;

/// `len` bytes from `rep + 1` back, which must lie inside the dictionary.
pub fn copy(out: &mut Vec<u8>, start: usize, rep: u32, len: usize, dict: u32) -> Option<()> {
    let back = (rep as usize).checked_add(1)?;
    if back > out.len() - start || rep >= dict {
        return None;
    }
    for _ in 0..len {
        out.push(out[out.len() - back]);
    }
    Some(())
}
