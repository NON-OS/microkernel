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

//! `audit --agent <name>`: one capsule, each area with the capabilities that
//! open it, and where that authority came from: the ceiling its signed
//! manifest declared, as the kernel recorded it at admission.

use alloc::vec::Vec;

use super::format::{hex, printable};
use super::join::Row;
use super::names::names;
use super::reach::{through, AREAS, DEVICES_NOTE};
use super::table::{list, pad, pid};
use super::withheld::holds;
use crate::command::builtin::receipt::own::signer;

fn one(out: &mut Vec<Vec<u8>>, row: &Row, name: &[u8]) {
    let mut head = printable(name);
    head.extend_from_slice(b", pid ");
    pid(&mut head, row.pid, 0);
    out.push(head);
    let Some(c) = row.ceiling else {
        out.push(Vec::from(&b"  not in the kernel's registry: no signed ceiling recorded"[..]));
        return;
    };
    let mut from = Vec::from(&b"  authority  its signed manifest's ceiling, signed by "[..]);
    from.extend_from_slice(signer(c.authority));
    from.extend_from_slice(b", measurement ");
    hex(&mut from, &c.measurement);
    out.push(from);
    for area in &AREAS {
        let mut line = Vec::from(&b"  "[..]);
        line.extend_from_slice(area.label);
        pad(&mut line, 13);
        let by = through(c.caps, area);
        line.extend_from_slice(if by.is_empty() { b"no" } else { b"YES, through " });
        list(&mut line, &by);
        out.push(line);
    }
    let mut all = Vec::from(&b"  holds      "[..]);
    list(&mut all, &names(c.caps));
    out.push(all);
    if row.debug_withheld {
        let mut d = Vec::from(&b"  debug      withheld by the kernel, as it "[..]);
        d.extend_from_slice(&holds(c.caps));
        out.push(d);
    }
}

/// The lines for every running capsule named `name`: one block each, as a
/// name can run more than once. Empty when none is.
pub fn agent(rows: &[Row], name: &[u8]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    for row in rows.iter().filter(|r| r.name.as_deref() == Some(name)) {
        one(&mut out, row, name);
    }
    if !out.is_empty() {
        out.push(Vec::from(DEVICES_NOTE));
    }
    out
}
