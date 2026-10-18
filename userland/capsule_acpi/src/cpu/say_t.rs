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

//! The throttling part of an `[ACPI] cpu` line: the states' range, the
//! limit _TPC sets, the domain, and the register space _PTC names.

use super::say_ids::space;
use crate::sys::log::Line;
use nonos_aml::cpu::CpuTables;

/// Throttling: only when firmware lists states, since most do not.
pub fn tstates(l: Line, t: &CpuTables) -> Line {
    let (Some(fast), Some(slow)) = (t.tstates.first(), t.tstates.last()) else { return l };
    let l = l.str(b"; T ").dec(t.tstates.len() as u64).str(b" ").dec(u64::from(fast.percent));
    let l = l.str(b"..").dec(u64::from(slow.percent)).str(b"%");
    let l = match t.tpc {
        Some(p) => l.str(b" _TPC ").dec(p),
        None => l,
    };
    let l = match t.tsd {
        Some(d) => l.str(b" domain ").dec(d.domain.into()),
        None => l,
    };
    match t.ptc {
        Some(p) => l.str(b" by ").str(space(p.control.space)),
        None => l.str(b" no _PTC"),
    }
}
