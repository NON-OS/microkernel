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

//! An FSE decoding table: per state, the symbol, how many bits the next
//! state reads, and what those bits are added to.

use alloc::vec;
use alloc::vec::Vec;

#[derive(Clone, Copy, Default)]
pub struct Cell {
    pub sym: u8,
    pub bits: u8,
    pub base: u16,
}

#[derive(Clone)]
pub struct Fse {
    pub log: u8,
    pub cells: Vec<Cell>,
}

impl Fse {
    /// A table that always yields `sym` and reads nothing: RLE mode.
    pub fn single(sym: u8) -> Fse {
        Fse { log: 0, cells: vec![Cell { sym, bits: 0, base: 0 }] }
    }
}
