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

use super::header::Header;

/* PLTE and tRNS as the pixels read them. tRNS gives palette alphas for
 * color type 3, and for types 0 and 2 the one grey or RGB value (compared
 * at the image bit depth) that is fully transparent. */
pub(super) struct Palette<'a> {
    pub plte: &'a [u8],
    pub trns: &'a [u8],
    pub key: [Option<u16>; 3],
}

impl<'a> Palette<'a> {
    pub fn new(h: &Header, plte: &'a [u8], trns: &'a [u8]) -> Self {
        let mask = if h.bit_depth == 16 { 0xffff } else { (1u16 << h.bit_depth) - 1 };
        let at =
            |i: usize| trns.get(2 * i..2 * i + 2).map(|b| u16::from_be_bytes([b[0], b[1]]) & mask);
        let key = match h.color_type {
            0 => [at(0), at(0), at(0)],
            2 if trns.len() >= 6 => [at(0), at(1), at(2)],
            _ => [None; 3],
        };
        Self { plte, trns, key }
    }
}
