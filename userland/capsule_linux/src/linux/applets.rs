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

//! The programs a BusyBox binary holds, read from the binary itself.
//!
//! BusyBox keeps its program names as one sorted run of NUL-terminated
//! strings, "[" and "[[" first, and reads argv[0] against that table to
//! choose what to be. Reading the same table here means the answer always
//! matches the bytes that would run, whatever config built them.

const FIRST: &[u8] = b"[\0[[\0";

/// The program names in `elf`'s BusyBox table, in order; none when there is
/// no table to read.
pub fn names(elf: &[u8]) -> impl Iterator<Item = &[u8]> {
    let at = elf.windows(FIRST.len()).position(|w| w == FIRST).unwrap_or(elf.len());
    elf[at..].split(|&b| b == 0).take_while(|name| !name.is_empty())
}

/// Whether `elf`'s BusyBox runs a program called `name`.
pub fn has(elf: &[u8], name: &[u8]) -> bool {
    !name.is_empty() && names(elf).any(|n| n == name)
}
