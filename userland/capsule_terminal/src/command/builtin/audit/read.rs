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

//! Reading the kernel's two tables for `audit`, and refusing, by name,
//! whichever it withholds or hands over broken.

use alloc::vec::Vec;

use nonos_libc::{PROC_FLAG_DEBUG_WITHHELD, PROC_NAME_LEN};

use super::join::{join, Proc, Row};
use crate::command::builtin::proc_table::table;
use crate::command::builtin::receipt::entries;

/// The joined rows, or the line that says which table the kernel withheld.
pub fn rows() -> Result<Vec<Row>, &'static [u8]> {
    let regs = entries().map_err(|_| &b"audit: the kernel did not hand over its registry"[..])?;
    let procs = table();
    if procs.is_empty() {
        return Err(b"audit: the kernel did not hand over its process table");
    }
    // The raw name bytes: a name that is not UTF-8 is still a capsule.
    let named: Vec<Proc<'_>> = procs
        .iter()
        .map(|p| {
            let name = &p.name[..(p.name_len as usize).min(PROC_NAME_LEN)];
            (p.pid, name, p.flags & PROC_FLAG_DEBUG_WITHHELD != 0)
        })
        .collect();
    join(&regs, &named).ok_or(b"audit: the kernel's registry is not whole entries; refused")
}
