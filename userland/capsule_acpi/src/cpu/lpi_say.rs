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

//! The _LPI part of an `[ACPI] cpu` line: each level, its own first, with
//! how many states it has and the span of their wake latencies.

use super::lpi_record::usable;
use crate::sys::log::Line;
use nonos_aml::cpu::{CpuTables, LpiLevel};

fn level(l: Line, lv: &LpiLevel) -> Line {
    let l = match lv.container {
        None => l.str(b"own "),
        Some((_, uid)) => l.str(b"container ").dec(uid).str(b" "),
    };
    let lat = lv.lpi.states.iter().map(|s| u64::from(s.wake_latency_us));
    let (lo, hi) = (lat.clone().min().unwrap_or(0), lat.max().unwrap_or(0));
    let l = l.dec(lv.lpi.states.len() as u64).str(b" states ").dec(lo);
    if lo == hi { l } else { l.str(b"..").dec(hi) }.str(b"us")
}

pub fn lpi(l: Line, t: &CpuTables) -> Line {
    if t.lpi.is_empty() {
        return l.str(b"no _LPI");
    }
    let l = t
        .lpi
        .iter()
        .enumerate()
        .fold(l.str(b"LPI "), |l, (i, lv)| level(if i == 0 { l } else { l.str(b", ") }, lv));
    match t.lpi.len() - usable(&t.lpi).len() {
        0 => l,
        n => l.str(b" (").dec(n as u64).str(b" not handed over: a register the kernel refuses)"),
    }
}
