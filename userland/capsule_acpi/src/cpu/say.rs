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

//! What each table says, for an `[ACPI] cpu` line.

use super::say_ids::space;
use crate::sys::log::Line;
use nonos_aml::cpu::*;

pub fn pstates(l: Line, t: &CpuTables) -> Line {
    let (Some(fast), Some(slow)) = (t.pstates.first(), t.pstates.last()) else {
        return l.str(b"no _PSS");
    };
    let l = l.str(b"P ").dec(t.pstates.len() as u64).str(b" ").dec(u64::from(fast.mhz));
    let l = l.str(b"..").dec(u64::from(slow.mhz)).str(b" MHz");
    let l = match t.ppc {
        Some(p) => l.str(b" _PPC ").dec(p),
        None => l,
    };
    match t.pct {
        Some(p) => l.str(b" by ").str(space(p.control.space)),
        None => l.str(b" no _PCT"),
    }
}

pub fn cstates(l: Line, t: &CpuTables) -> Line {
    if t.cstates.is_empty() {
        return l.str(b"no _CST");
    }
    t.cstates.iter().enumerate().fold(l, |l, (i, c)| {
        let l = if i == 0 { l } else { l.str(b" ") };
        let l = l.str(b"C").dec(u64::from(c.kind)).str(b" ").dec(u64::from(c.latency_us));
        l.str(b"us ").str(space(c.register.space))
    })
}

pub fn cpc(l: Line, t: &CpuTables) -> Line {
    let Some(c) = t.cpc else { return l.str(b"no _CPC") };
    let level = |l: Line, name: &[u8], i: usize| match c.integer(i) {
        Some(v) => l.str(name).dec(v),
        None => l,
    };
    let l = l.str(b"CPPC rev ").dec(u64::from(c.revision));
    let l = level(l, b" highest ", HIGHEST);
    let l = level(l, b" nominal ", NOMINAL);
    level(l, b" lowest ", LOWEST)
}
