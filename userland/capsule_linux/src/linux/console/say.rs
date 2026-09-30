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
//! or said, only numbers and names this capsule chose.

use nonos_libc::mk_private_write;

use super::state::attached;

/// The most the kernel takes in one private write.
const PIECE: usize = 256;

pub fn say(line: &[u8]) {
    if !attached() {
        return;
    }
    for piece in line.chunks(PIECE) {
        if mk_private_write(piece) < 0 {
            return;
        }
    }
}
