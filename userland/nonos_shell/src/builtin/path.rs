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

//! `basename PATH [SUFFIX]` and `dirname PATH`: the last part of a path and
//! the part before it, by the path's text alone, as the POSIX utilities
//! read it. Trailing slashes are not a part; `/` is its own last part and
//! its own directory; a path with no slash lives in `.`.

use crate::out::ShellOut;

pub fn basename(out: &mut dyn ShellOut, argv: &[&[u8]]) -> bool {
    match argv {
        [_, path] => out.writeln(base(path, b"")),
        [_, path, suffix] => out.writeln(base(path, suffix)),
        _ => {
            out.writeln(b"usage: basename path [suffix]");
            return false;
        }
    }
    true
}

pub fn dirname(out: &mut dyn ShellOut, argv: &[&[u8]]) -> bool {
    match argv {
        [_, path] => out.writeln(dir(path)),
        _ => {
            out.writeln(b"usage: dirname path");
            return false;
        }
    }
    true
}

fn trim_slashes(p: &[u8]) -> &[u8] {
    let end = p.iter().rposition(|c| *c != b'/').map_or(0, |i| i + 1);
    &p[..end]
}

/// The last part of `path`, less `suffix` unless that is all of it.
pub fn base<'a>(path: &'a [u8], suffix: &[u8]) -> &'a [u8] {
    if path.is_empty() {
        return b"";
    }
    let t = trim_slashes(path);
    if t.is_empty() {
        return b"/";
    }
    let last = t.iter().rposition(|c| *c == b'/').map_or(t, |i| &t[i + 1..]);
    match last.strip_suffix(suffix) {
        Some(s) if !s.is_empty() => s,
        _ => last,
    }
}

/// Everything in `path` before its last part.
pub fn dir(path: &[u8]) -> &[u8] {
    let t = trim_slashes(path);
    if t.is_empty() {
        return if path.is_empty() { b"." } else { b"/" };
    }
    match t.iter().rposition(|c| *c == b'/') {
        None => b".",
        Some(i) => {
            let d = trim_slashes(&t[..i]);
            if d.is_empty() {
                b"/"
            } else {
                d
            }
        }
    }
}
