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

//! `ps`: the machine's processes, one line each, as the terminal's `ps`
//! prints them: pid, parent, state and name. A name is the kernel's, but a
//! Linux guest names its own processes, so every byte outside printable
//! ASCII is shown as `?`: no name reaches the terminal at the other end of
//! a session as a control sequence.

use alloc::vec::Vec;

use crate::host::{Host, Proc};
use crate::out::ShellOut;
use crate::util::format_u64;

pub const HEADER: &[u8] = b"   pid  ppid  state     name";
pub const UNAVAILABLE: &[u8] = b"ps: the kernel did not hand over its process table";

pub fn run(out: &mut dyn ShellOut, host: &mut dyn Host, w: &[&[u8]]) -> bool {
    if w.len() > 1 {
        out.writeln(b"usage: ps");
        return false;
    }
    let Some(procs) = host.procs().filter(|p| !p.is_empty()) else {
        out.writeln(UNAVAILABLE);
        return false;
    };
    out.writeln(HEADER);
    for p in procs.iter().filter(|p| p.pid != 0) {
        out.writeln(&line(p));
    }
    true
}

fn line(p: &Proc) -> Vec<u8> {
    let mut l = Vec::with_capacity(32 + p.name.len());
    col(&mut l, p.pid, 6);
    col(&mut l, p.ppid, 6);
    l.extend_from_slice(b"  ");
    let state = state_word(p.state);
    l.extend_from_slice(state);
    l.resize(l.len() + 10usize.saturating_sub(state.len()), b' ');
    l.extend(p.name.iter().map(|&c| if (0x20..0x7f).contains(&c) { c } else { b'?' }));
    l
}

fn col(l: &mut Vec<u8>, v: u32, width: usize) {
    let mut num = [0u8; 20];
    let n = format_u64(u64::from(v), &mut num);
    l.resize(l.len() + width.saturating_sub(n), b' ');
    l.extend_from_slice(&num[..n]);
}

/// The kernel's state numbers, as the terminal names them.
pub fn state_word(state: u8) -> &'static [u8] {
    match state {
        0 => b"new",
        1 => b"ready",
        2 => b"running",
        3 => b"idle",
        4 => b"stopped",
        5 => b"zombie",
        _ => b"exited",
    }
}
