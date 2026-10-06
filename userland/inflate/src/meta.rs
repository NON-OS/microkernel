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

//! What each symbol of the three DEFLATE alphabets decodes to, folded into
//! its table entry so the hot loop needs no second lookup.

use super::huff::{BAD, EOB, LEN, LIT};
use super::tables::{DBASE, DEXT, LBASE, LEXT};

/// Literal/length alphabet: bytes, end of block, lengths 3..=258.
/// Symbols 286 and 287 exist in the fixed code but are invalid.
pub fn litlen(sym: usize) -> u32 {
    match sym {
        0..=255 => LIT | (sym as u32) << 16,
        256 => EOB,
        257..=285 => {
            let s = sym - 257;
            LEN | u32::from(LBASE[s]) << 16 | u32::from(LEXT[s]) << 8
        }
        _ => BAD,
    }
}

/// Distance alphabet; symbols 30 and 31 are invalid.
pub fn dist(sym: usize) -> u32 {
    match (DBASE.get(sym), DEXT.get(sym)) {
        (Some(&base), Some(&ext)) => LIT | u32::from(base) << 16 | u32::from(ext) << 8,
        _ => BAD,
    }
}

/// Code-length alphabet: the symbol itself.
pub fn plain(sym: usize) -> u32 {
    LIT | (sym as u32) << 16
}
