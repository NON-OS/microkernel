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

//! mprotect of a span that holds shared file maps.

use super::asks::{protects, writers, PROT_EXEC, PROT_WRITE};
use super::back::too_long;
use super::entry::overlap;
use super::table::{picked, with};
use crate::linux::abi::errno;

/// mprotect of `[at, at + span)` of `pid` to `prot`, as asks.rs allows it,
/// and ENOSYS for write to a file too long to write back (back.rs). A map
/// given write is a writer from here on.
pub fn protect(pid: u32, at: u64, span: u64, prot: u64) -> Result<(), i64> {
    let (end, write) = (at.saturating_add(span), prot & PROT_WRITE != 0);
    with(|all| protects(all, pid, at, end, (write, prot & PROT_EXEC != 0)))?;
    if !write {
        return Ok(());
    }
    let hit = picked(|e| e.pid == pid && !e.orphan && overlap(e.place(), (at, end)).is_some());
    if hit.iter().any(|e| too_long(&e.path)) {
        return Err(errno::ENOSYS);
    }
    with(|all| writers(all, pid, at, end));
    Ok(())
}
