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

//! One row of the vendor table, from a line and the program's file.

use std::fs;
use std::path::PathBuf;

use super::line::Line;
use super::source::Row;
use crate::dests::dests_hash;
use crate::mask::{grants, NET_ANON};
use crate::policy::LinuxAppPolicy;
use crate::unlisted::unlisted;

/// A list is reached only through NET_ANON, so a line naming one without it
/// is a mistake the build says, not a box with a list nothing reads.
pub fn add(rows: &mut Vec<Row>, l: &Line<'_>, at: PathBuf) -> Result<(), String> {
    if l.dests.is_some() && !grants(l.mask, NET_ANON) {
        return Err(format!("{}: dests= without NET_ANON", l.store_path));
    }
    println!("cargo:rerun-if-changed={}", at.display());
    let elf = fs::read(&at).map_err(|e| format!("{}: {e}", at.display()))?;
    let program = *blake3::hash(&elf).as_bytes();
    let dests = l.dests.clone().unwrap_or_default();
    let net_dests_hash = l.dests.as_deref().map_or([0; 32], dests_hash);
    let policy =
        LinuxAppPolicy { mask: l.mask, syscall_set: l.calls, net_dests_hash, ..unlisted(program) };
    let row = (l.store_path.to_string(), program, policy.encode(), dests);
    if rows.iter().any(|r| r.0 == row.0 || (r.1 == row.1 && r.2 != row.2)) {
        return Err(format!("{}: named twice, or one program in two boxes", row.0));
    }
    rows.push(row);
    Ok(())
}
