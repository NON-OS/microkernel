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

//! `ps`: every process in the kernel's table, with its parent and state. It
//! printed the fixed service list `capsules` used to, which was neither every
//! process nor anything about them but whether a name was registered.

use alloc::vec::Vec;

use super::proc_table::{state_word, table};
use crate::command::output::Output;
use crate::term::util::format_u64;

fn col(line: &mut Vec<u8>, v: u32, width: usize) {
    let mut num = [0u8; 12];
    let n = format_u64(v as u64, &mut num);
    line.resize(line.len() + width.saturating_sub(n), b' ');
    line.extend_from_slice(&num[..n]);
}

pub fn run(out: &mut Output<'_>) {
    let procs = table();
    if procs.is_empty() {
        out.writeln(b"ps: the kernel did not hand over its process table");
        return;
    }
    out.writeln(b"   pid  ppid  state     name");
    for p in procs.iter().filter(|p| p.pid != 0) {
        let mut line = Vec::with_capacity(64);
        col(&mut line, p.pid, 6);
        col(&mut line, p.ppid, 6);
        line.extend_from_slice(b"  ");
        let state = state_word(p.state);
        line.extend_from_slice(state);
        line.resize(line.len() + 10usize.saturating_sub(state.len()), b' ');
        line.extend_from_slice(p.name_str().as_bytes());
        out.writeln(&line);
    }
}
