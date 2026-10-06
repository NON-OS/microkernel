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

//! What one frame's blocks share: the last Huffman table, the last sequence
//! tables for Repeat mode, and the three repeat offsets.

use super::fse_build::Fse;
use super::huff_build::Huff;

pub struct Context {
    pub huff: Option<Huff>,
    pub ll: Option<Fse>,
    pub of: Option<Fse>,
    pub ml: Option<Fse>,
    pub rep: [usize; 3],
}

impl Context {
    pub fn new() -> Self {
        Context { huff: None, ll: None, of: None, ml: None, rep: [1, 4, 8] }
    }
}
