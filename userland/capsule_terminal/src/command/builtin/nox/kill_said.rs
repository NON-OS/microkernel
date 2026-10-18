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

//! The line a kill that landed prints.

use alloc::vec::Vec;

use crate::term::util::format_u64;

/// "killed", the name when one was given, and the pid.
pub(super) fn killed(pid: u64, name: Option<&[u8]>) -> Vec<u8> {
    let mut line: Vec<u8> = Vec::new();
    line.extend_from_slice(b"killed ");
    if let Some(name) = name {
        line.extend_from_slice(name);
        line.push(b' ');
    }
    line.extend_from_slice(b"pid ");
    append_u64(&mut line, pid);
    line
}

fn append_u64(out: &mut Vec<u8>, v: u64) {
    let mut buf = [0u8; 24];
    let k = format_u64(v, &mut buf);
    out.extend_from_slice(&buf[..k]);
}
