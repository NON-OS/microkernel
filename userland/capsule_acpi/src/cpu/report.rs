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

//! One `[ACPI] cpu` line for each run of processors with the same tables:
//! a package's cores usually share them, and a hybrid laptop's two kinds
//! of core then show as two lines.

use super::lpi_say::lpi;
use super::say::{cpc, cstates, pstates};
use super::say_ids::uids;
use super::state::Cpus;
use crate::sys::log::Line;
use alloc::vec::Vec;
use nonos_aml::cpu::CpuTables;

pub fn report(cpus: &Cpus) {
    if cpus.list.is_empty() {
        Line::new(b"cpu: no Processor objects or ACPI0007 devices").end();
        return;
    }
    let mut start = 0;
    while start < cpus.list.len() {
        let t = &cpus.list[start].tables;
        let run = cpus.list[start..].iter().take_while(|c| c.tables == *t).count();
        let ids: Vec<u64> = cpus.list[start..start + run].iter().map(|c| c.uid).collect();
        line(&ids, t);
        start += run;
    }
}

pub fn line(ids: &[u64], t: &CpuTables) {
    let l = uids(Line::new(b"cpu "), ids).str(b": ");
    let l = pstates(l, t).str(b"; ");
    let l = cstates(l, t).str(b"; ");
    let l = cpc(l, t).str(b"; ");
    lpi(l, t).end();
}
