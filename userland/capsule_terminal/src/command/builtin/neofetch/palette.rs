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

//! The two ANSI colour strips printed under the info column: the eight
//! colours, then their bright forms, as fastfetch shows them.

use alloc::vec::Vec;

const CODES: [[&str; 8]; 2] = [
    [
        "\x1b[30m", "\x1b[31m", "\x1b[32m", "\x1b[33m", "\x1b[34m", "\x1b[35m", "\x1b[36m",
        "\x1b[37m",
    ],
    [
        "\x1b[90m", "\x1b[91m", "\x1b[92m", "\x1b[93m", "\x1b[94m", "\x1b[95m", "\x1b[96m",
        "\x1b[97m",
    ],
];

const BLOCK: &str = "██";

/// Strip `row` (0 the colours, 1 the bright ones) as a `(plain, styled)` pair.
///
/// `styled` carries one colour escape per block and is what the grid renders;
/// `plain` is the same blocks with no escapes at all, so a redirected
/// `neofetch` writes glyphs to the file rather than terminal control codes.
pub fn palette(row: usize) -> (Vec<u8>, Vec<u8>) {
    let codes = &CODES[row.min(1)];
    let mut plain = Vec::with_capacity(codes.len() * BLOCK.len());
    let mut styled = Vec::with_capacity(codes.len() * (BLOCK.len() + 5));
    for code in codes {
        styled.extend_from_slice(code.as_bytes());
        styled.extend_from_slice(BLOCK.as_bytes());
        plain.extend_from_slice(BLOCK.as_bytes());
    }
    styled.extend_from_slice(b"\x1b[0m");
    (plain, styled)
}
