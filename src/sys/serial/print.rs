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

use super::core::{with_serial_lock, write_byte};

fn emit(s: &[u8]) {
    for &ch in s {
        write_byte(ch);
    }
}

pub fn print(s: &[u8]) {
    with_serial_lock(|| emit(s));
}

/// A process's own bytes (MkDebug, stdout), in one write as it gave them; a
/// line it splits across calls is its own (column.rs keeps the kernel's whole).
pub fn passthrough(s: &[u8]) {
    print(s);
}

/// A whole line, begun at column 0 (column.rs), in one write under the lock.
pub fn line(s: &[u8]) {
    with_serial_lock(|| {
        emit(super::column::opening());
        emit(s);
        emit(b"\r\n");
    });
}

pub fn println(s: &[u8]) {
    with_serial_lock(|| {
        emit(s);
        emit(b"\r\n");
    });
}
