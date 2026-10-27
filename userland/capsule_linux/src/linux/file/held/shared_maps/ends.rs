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

//! A shared map ending: unmapped in part or whole, or its process gone. A
//! writer's pages go back to the file before its pages go.

use alloc::vec::Vec;

use super::super::cache;
use super::back::back;
use super::rules::unmap;
use super::spans::going;
use super::table::{picked, with};

/// `[at, at + span)` of `pid` unmapped: what writers changed there goes
/// back, then the maps there are cut around the hole or split by it.
/// Nothing is done when the split is refused (rules.rs).
pub fn unmapped(pid: u32, at: u64, span: u64) -> Result<(), i64> {
    let gone = picked(|e| e.pid == pid && going(e, at, span).is_some());
    if gone.is_empty() {
        return Ok(());
    }
    let kept = with(|all| unmap(all, pid, at, span))?;
    for (e, part) in gone.iter().filter_map(|e| Some((e, going(e, at, span)?))) {
        back(e, part);
    }
    with(|all| *all = kept);
    Ok(())
}

/// `pid` exec'd or ended: its writers' pages go back while they are still
/// there, its maps leave the table, and each file they changed goes to the
/// store now, since a process killed by a signal flushes nothing itself.
pub fn ended(pid: u32) {
    let mut changed: Vec<Vec<u8>> = Vec::new();
    for e in picked(|e| e.pid == pid) {
        if back(&e, e.file()) && !changed.contains(&e.path) {
            changed.push(e.path);
        }
    }
    with(|all| all.retain(|e| e.pid != pid));
    for path in changed {
        let _ = cache::flush(&path, true);
    }
}

/// Whether `[at, at + span)` of `pid` holds a shared map, which mremap
/// refuses with ENOSYS: moved or grown, a map would show no file there.
pub fn holds(pid: u32, at: u64, span: u64) -> bool {
    with(|all| all.iter().any(|e| e.pid == pid && going(e, at, span).is_some()))
}
