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
use super::format::HEADING;
use super::format::{parse, Mode, FORMAT};
use super::reach::{reaches, AREAS, DEVICES_NOTE};
use super::read::rows;
use super::table::line;
use crate::command::builtin::receipt::count_line;
use crate::command::output::Output;

pub fn run(out: &mut Output<'_>, argv: &[&[u8]]) {
    out.writeln(FORMAT);
    let mode = match parse(argv.get(1..).unwrap_or(&[])) {
        Ok(m) => m,
        Err(e) => return out.writeln(&e),
    };
    let rows = match rows() {
        Ok(r) => r,
        Err(e) => return out.writeln(e),
    };
    if let Mode::Agent(name) = mode {
        let lines = agent(&rows, name);
        if lines.is_empty() {
            out.writeln(b"audit: no running capsule has that name; `audit` lists them");
        }
        return lines.iter().for_each(|l| out.writeln(l));
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
