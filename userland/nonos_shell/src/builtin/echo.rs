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

use crate::out::ShellOut;

/// The longest line `echo` builds, the terminal's line width.
pub const LINE_MAX: usize = 1024;

pub fn run(out: &mut dyn ShellOut, argv: &[&[u8]]) {
    if argv.len() <= 1 {
        out.writeln(b"");
        return;
    }
    let mut buf = [0u8; LINE_MAX];
    let mut n = 0;
    for (i, arg) in argv[1..].iter().enumerate() {
        if i > 0 && n < buf.len() {
            buf[n] = b' ';
            n += 1;
        }
        n += copy_into(&mut buf[n..], arg);
        if n == buf.len() {
            break;
        }
    }
    out.writeln(&buf[..n]);
}

/// Copy as much of `src` as fits into `dst`; the count copied.
fn copy_into(dst: &mut [u8], src: &[u8]) -> usize {
    let n = src.len().min(dst.len());
    dst[..n].copy_from_slice(&src[..n]);
    n
}
