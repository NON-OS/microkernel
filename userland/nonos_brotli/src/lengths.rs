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

use crate::bits::Bits;
use crate::error::Error;
use crate::huff::Code;

/// Insert and copy lengths: base and extra bits per code (RFC 7932 5).
const INSERT_BASE: [u32; 24] = [
    0, 1, 2, 3, 4, 5, 6, 8, 10, 14, 18, 26, 34, 50, 66, 98, 130, 194, 322, 578, 1090, 2114, 6210,
    22594,
];
const INSERT_EXTRA: [u8; 24] =
    [0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 7, 8, 9, 10, 12, 14, 24];
const COPY_BASE: [u32; 24] = [
    2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 18, 22, 30, 38, 54, 70, 102, 134, 198, 326, 582, 1094, 2118,
];
const COPY_EXTRA: [u8; 24] =
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 7, 8, 9, 10, 24];
/// The first insert and copy code of each 64-symbol cell.
const CELL_INSERT: [usize; 11] = [0, 0, 0, 0, 8, 8, 0, 16, 8, 16, 16];
const CELL_COPY: [usize; 11] = [0, 8, 0, 8, 0, 8, 16, 0, 16, 8, 16];

/// Block counts: base and extra bits per symbol (RFC 7932 6).
const BASE: [u32; 26] = [
    1, 5, 9, 13, 17, 25, 33, 41, 49, 65, 81, 97, 113, 145, 177, 209, 241, 305, 369, 497, 753, 1265,
    2289, 4337, 8433, 16625,
];
const EXTRA: [u8; 26] =
    [2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 7, 8, 9, 10, 11, 12, 13, 24];

/// One command: literals to insert, then a copy.
pub(crate) struct Command {
    pub(crate) insert: usize,
    pub(crate) copy: usize,
    /// The copy reuses the last distance and codes none of its own.
    pub(crate) same_distance: bool,
}

pub(crate) fn read_command(b: &mut Bits, code: &Code) -> Result<Command, Error> {
    let c = code.read(b)? as usize;
    let (ic, cc) = (CELL_INSERT[c >> 6] + (c >> 3 & 7), CELL_COPY[c >> 6] + (c & 7));
    let insert = INSERT_BASE[ic] + b.read(INSERT_EXTRA[ic] as u32)?;
    let copy = COPY_BASE[cc] + b.read(COPY_EXTRA[cc] as u32)?;
    Ok(Command { insert: insert as usize, copy: copy as usize, same_distance: c < 128 })
}

/// A block count: a symbol of `code`, then its extra bits.
pub(crate) fn block_count(b: &mut Bits, code: &Code) -> Result<u32, Error> {
    let s = code.read(b)? as usize;
    Ok(BASE[s] + b.read(EXTRA[s] as u32)?)
}
