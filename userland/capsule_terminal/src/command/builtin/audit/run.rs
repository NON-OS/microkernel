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

//! Reading the kernel's two tables, joining them, and printing.

use alloc::vec::Vec;

use super::agent::agent;
use super::join::{join, Row};
use super::reach::{reaches, AREAS, DEVICES_NOTE};
use super::table::{line, HEADING};
use crate::command::builtin::proc_table::table;
use crate::command::builtin::receipt::{count_line, entries};
use crate::command::output::Output;

fn rows() -> Option<Vec<Row>> {
    let procs = table();
    let named: Vec<(u32, &[u8])> = procs.iter().map(|p| (p.pid, p.name_str().as_bytes())).collect();
    let regs = entries().ok()?;
    Some(join(&regs, &named))
}

pub fn run(out: &mut Output<'_>, argv: &[&[u8]]) {
    let Some(rows) = rows() else {
        out.writeln(b"the kernel did not hand over its registry");
        return;
    };
    if let Some(i) = argv.iter().position(|a| *a == b"--agent") {
        let Some(name) = argv.get(i + 1) else {
            out.writeln(b"usage: audit --agent <name>");
            return;
        };
        let lines = agent(&rows, name);
        if lines.is_empty() {
            out.writeln(b"no running capsule has that name; `audit` lists them");
        }
        lines.iter().for_each(|l| out.writeln(l));
        return;
    }
    HEADING.iter().for_each(|h| out.writeln(h));
    rows.iter().for_each(|r| out.writeln(&line(r)));
    out.writeln(b"");
    let signed: Vec<u64> = rows.iter().filter_map(|r| r.ceiling.map(|c| c.caps)).collect();
    for area in &AREAS {
        let n = signed.iter().filter(|c| reaches(**c, area)).count();
        out.writeln(&count_line(n, signed.len(), area.said));
    }
    if rows.len() > signed.len() {
        let mut l = count_line(rows.len() - signed.len(), rows.len(), b"have no signed ceiling");
        l.extend_from_slice(b" Their reach is not shown.");
        out.writeln(&l);
    }
    out.writeln(DEVICES_NOTE);
}
