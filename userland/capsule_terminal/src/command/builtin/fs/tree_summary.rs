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

//! The closing line of a tree: how many directories and files it drew.

use alloc::vec::Vec;

/// "N directories, M files", built without a formatter.
pub(super) fn summary(dirs: usize, files: usize) -> Vec<u8> {
    let mut line = Vec::new();
    push_num(&mut line, dirs);
    line.extend_from_slice(if dirs == 1 { b" directory, " } else { b" directories, " });
    push_num(&mut line, files);
    line.extend_from_slice(if files == 1 { b" file" } else { b" files" });
    line
}

fn push_num(out: &mut Vec<u8>, mut n: usize) {
    if n == 0 {
        out.push(b'0');
        return;
    }
    let mut digits = [0u8; 20];
    let mut i = digits.len();
    while n > 0 {
        i -= 1;
        digits[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    out.extend_from_slice(&digits[i..]);
}
