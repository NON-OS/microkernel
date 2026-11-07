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

//! `diff A B`: the lines that differ between two files, `< ` for a line
//! only in A and `> ` for a line only in B, in file order, by the longest
//! common run of lines. Equal files print nothing and succeed; files that
//! differ fail, as the POSIX utility's status does. Each file is held to
//! `MAX_BYTES` and `MAX_LINES`, so the table the comparison builds stays
//! small.

use alloc::vec;
use alloc::vec::Vec;

use crate::host::Host;
use crate::out::ShellOut;

pub const MAX_BYTES: usize = 64 * 1024;
pub const MAX_LINES: usize = 512;

pub fn run(out: &mut dyn ShellOut, host: &mut dyn Host, argv: &[&[u8]]) -> bool {
    let [_, a, b] = argv else {
        out.writeln(b"usage: diff file1 file2");
        return false;
    };
    let (Some(left), Some(right)) = (lines(out, host, a), lines(out, host, b)) else {
        return false;
    };
    let edits = compare(&left, &right);
    for (mark, line) in &edits {
        let mut l = vec![*mark, b' '];
        l.extend_from_slice(line);
        out.writeln(&l);
    }
    edits.is_empty()
}

fn lines(out: &mut dyn ShellOut, host: &mut dyn Host, path: &[u8]) -> Option<Vec<Vec<u8>>> {
    let Some(data) = host.read_file(path, MAX_BYTES) else {
        refuse(out, path, b": cannot read it, or it is over 64 KiB");
        return None;
    };
    let mut v: Vec<Vec<u8>> = data.split(|c| *c == b'\n').map(<[u8]>::to_vec).collect();
    if data.last() == Some(&b'\n') {
        v.pop();
    }
    if v.len() > MAX_LINES {
        refuse(out, path, b": over 512 lines");
        return None;
    }
    Some(v)
}

fn refuse(out: &mut dyn ShellOut, path: &[u8], why: &[u8]) {
    let mut l = b"diff: ".to_vec();
    l.extend_from_slice(path);
    l.extend_from_slice(why);
    out.writeln(&l);
}

/// The edits from `a` to `b`, removals before additions at each point. Both
/// are held to `MAX_LINES`, so no count in the table passes `u16`.
fn compare<'a>(a: &'a [Vec<u8>], b: &'a [Vec<u8>]) -> Vec<(u8, &'a [u8])> {
    let w = b.len() + 1;
    let mut t = vec![0u16; (a.len() + 1) * w];
    for i in (0..a.len()).rev() {
        for j in (0..b.len()).rev() {
            t[i * w + j] = if a[i] == b[j] {
                t[(i + 1) * w + j + 1] + 1
            } else {
                t[(i + 1) * w + j].max(t[i * w + j + 1])
            };
        }
    }
    let (mut i, mut j, mut out) = (0, 0, Vec::new());
    while i < a.len() || j < b.len() {
        if i < a.len() && j < b.len() && a[i] == b[j] {
            (i, j) = (i + 1, j + 1);
        } else if j == b.len() || (i < a.len() && t[(i + 1) * w + j] >= t[i * w + j + 1]) {
            out.push((b'<', &a[i][..]));
            i += 1;
        } else {
            out.push((b'>', &b[j][..]));
            j += 1;
        }
    }
    out
}
