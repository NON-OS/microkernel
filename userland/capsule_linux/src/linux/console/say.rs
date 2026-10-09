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

//! A line the personality itself says to the person at the terminal, never
//! to a log. Said only on a terminal; it is never anything a guest was told
//! or said, only numbers and names this capsule chose. When stdout goes to
//! a file it is said as stderr (`TAG_STDERR`), so it stays on the screen.

use nonos_libc::{mk_private_write, TAG_STDERR};

use super::state::{attached, split};

/// The most the kernel takes in one private write.
const PIECE: usize = 256;

pub fn say(line: &[u8]) {
    if !attached() {
        return;
    }
    let tagged = split();
    let mut framed = [0u8; PIECE];
    framed[0] = TAG_STDERR;
    for piece in line.chunks(PIECE - usize::from(tagged)) {
        let msg = match tagged {
            true => {
                framed[1..=piece.len()].copy_from_slice(piece);
                &framed[..=piece.len()]
            }
            false => piece,
        };
        if mk_private_write(msg) < 0 {
            return;
        }
    }
}
