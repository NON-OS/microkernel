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

//! LZMA's adaptive probabilities, and the state they are read in.

use alloc::vec::Vec;

use super::lzma_len::Len;

pub const HALF: u16 = 1024;

pub struct Model {
    pub lc: u32,
    pub lp: u32,
    pub pb: u32,
    pub state: usize,
    pub rep: [u32; 4],
    pub is_match: [u16; 192],
    pub is_rep: [[u16; 12]; 4],
    pub is_rep0_long: [u16; 192],
    pub pos_slot: [[u16; 64]; 4],
    pub pos: [u16; 115],
    pub align: [u16; 16],
    pub len: Len,
    pub rep_len: Len,
    pub literal: Vec<u16>,
}
