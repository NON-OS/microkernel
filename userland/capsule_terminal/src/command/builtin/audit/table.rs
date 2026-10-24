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

//! One line per capsule: pid, name, reach, signer, every capability.

use alloc::vec::Vec;

use super::format::{printable, NAME_W};
use super::join::Row;
use super::names::names;
use super::reach::{reaches, AREAS};
use super::withheld::note;
use crate::command::builtin::receipt::own::signer;
use crate::term::util::format_u64;

pub fn pad(line: &mut Vec<u8>, to: usize) {
    line.resize(line.len().max(to), b' ');
}

pub fn pid(line: &mut Vec<u8>, pid: u32, width: usize) {
    let mut num = [0u8; 12];
    let n = format_u64(pid as u64, &mut num);
    line.resize(line.len() + width.saturating_sub(n), b' ');
    line.extend_from_slice(&num[..n]);
}

pub fn name(row: &Row) -> Vec<u8> {
    printable(row.name.as_deref().unwrap_or(b"(exited)"))
}

pub fn list<T: AsRef<[u8]>>(line: &mut Vec<u8>, caps: &[T]) {
    for (i, c) in caps.iter().enumerate() {
        line.extend_from_slice(if i == 0 { b"" } else { b", " });
        line.extend_from_slice(c.as_ref());
    }
}

pub fn line(row: &Row) -> Vec<u8> {
    let mut line = Vec::with_capacity(128);
    pid(&mut line, row.pid, 5);
    line.extend_from_slice(b"  ");
    let shown = name(row);
    line.extend_from_slice(&shown[..shown.len().min(NAME_W - 2)]);
    let Some(c) = row.ceiling else {
        pad(&mut line, 7 + NAME_W);
        line.extend_from_slice(b"not in the kernel's registry: no signed ceiling recorded");
        return line;
    };
    for (i, area) in AREAS.iter().enumerate() {
        pad(&mut line, 7 + NAME_W + 9 * i);
        line.extend_from_slice(if reaches(c.caps, area) { b"YES" } else { b"no" });
    }
    pad(&mut line, 7 + NAME_W + 28);
    line.extend_from_slice(signer(c.authority));
    pad(&mut line, 7 + NAME_W + 39);
    list(&mut line, &names(c.caps));
    if row.debug_withheld {
        line.extend_from_slice(b"; ");
        line.extend_from_slice(&note(c.caps));
    }
    line
}
