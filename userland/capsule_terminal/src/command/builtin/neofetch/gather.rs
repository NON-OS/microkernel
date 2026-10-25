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

//! Reading what `neofetch` says: the kernel's process table, as the side rail
//! reads it, and this terminal's own grid and theme.

use alloc::vec::Vec;

use nonos_libc::mk_uptime_ms;

use super::facts;
use crate::rail::metrics::{Sample, MAX_PROCS};
use crate::term::state::State;
use crate::term::util::{copy_into, format_u64};

const VERSION: &str = include_str!("../../../../../../VERSION");

/// The figures, owned, for `run_info::Facts` to borrow.
pub(super) struct Read {
    pub kernel: Vec<u8>,
    pub uptime: Vec<u8>,
    pub memory: Vec<u8>,
    pub processes: Vec<u8>,
    pub load: Vec<u8>,
    pub terminal: Vec<u8>,
    pub theme: &'static [u8],
}

const THEMES: [&[u8]; 4] = [b"dark", b"dim", b"light", b"abyss"];

pub(super) fn read(state: &State) -> Read {
    let s = crate::rail::sample::poll(&Sample::EMPTY);
    let vt = &state.scrollback.vt;
    let theme = state.scrollback.theme_of.and_then(|i| THEMES.get(i as usize)).copied();
    let mut up = [0u8; 32];
    let ms = mk_uptime_ms();
    let n = if ms < 0 {
        copy_into(&mut up, b"unavailable")
    } else {
        crate::paint::fetch_uptime::uptime_str(ms as u64, &mut up)
    };
    Read {
        kernel: [b"microkernel ", VERSION.trim_end().as_bytes()].concat(),
        uptime: up[..n].to_vec(),
        memory: facts::memory(s.mem.used_kb.value(), s.mem.total_kb.value()),
        processes: count(s.n, s.n >= MAX_PROCS),
        load: facts::load(s.load_avg.value()),
        terminal: [count(vt.cols(), false), b"x".to_vec(), count(vt.rows(), false)].concat(),
        theme: theme.unwrap_or(b"unknown"),
    }
}

// `n`, or `n or more` when the table was full and may hold more.
fn count(n: usize, full: bool) -> Vec<u8> {
    let mut d = [0u8; 20];
    let k = format_u64(n as u64, &mut d);
    let mut out = d[..k].to_vec();
    if full {
        out.extend_from_slice(b" or more");
    }
    out
}
