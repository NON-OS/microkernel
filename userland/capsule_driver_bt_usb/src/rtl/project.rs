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

//! The extension block that ends a patch file: the signature 51 04 FD 77
//! last, and before it instructions read from the end backwards, each its
//! payload, then its length, then its opcode. Opcode 0 with one octet names
//! the project the file is for; opcode FF ends the block.

use super::epatch::HEADER;

pub const EXTENSION: [u8; 4] = [0x51, 0x04, 0xfd, 0x77];

/// The project the file names, if its extension block names one.
pub fn project(file: &[u8]) -> Option<u8> {
    let mut end = file.len().checked_sub(EXTENSION.len())?;
    if file[end..] != EXTENSION {
        return None;
    }
    // Instructions are read no lower than the header, which they never
    // overlap; each is at least an opcode, a length and one octet.
    while end >= HEADER + 3 {
        let (opcode, len) = (file[end - 1], usize::from(file[end - 2]));
        if opcode == 0xff || len == 0 {
            return None;
        }
        if opcode == 0 && len == 1 {
            return Some(file[end - 3]);
        }
        end = end.checked_sub(2 + len)?;
    }
    None
}
