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

//! What mmap and mprotect may ask of a shared file map. No page of one is
//! ever executable: its file can change after it was proved. Pure.

use super::entry::{overlap, Entry};
use super::rules::conflict;
use crate::linux::abi::errno;

/// mmap's and mprotect's bits, as Linux numbers them.
pub const PROT_WRITE: u64 = 0x2;
pub const PROT_EXEC: u64 = 0x4;

/// What mmap refuses of a descriptor mapped shared, in Linux's order:
/// ENODEV for anything but a file the store keeps, EACCES for one not open
/// to read, for write asked of one not open to write, and for exec.
pub fn refused(stored: bool, reads: bool, writable: bool, write: bool, exec: bool) -> Option<i64> {
    if !stored {
        return Some(errno::ENODEV);
    }
    (!reads || (write && !writable) || exec).then_some(errno::EACCES)
}

/// Whether `pid` may give `[at, end)` write or exec: EACCES for exec on a
/// map there, or for write through a descriptor not open to write; ENOSYS
/// for a map whose becoming a writer would break the one writer rule.
pub fn protects(all: &[Entry], pid: u32, at: u64, end: u64, prot: (bool, bool)) -> Result<(), i64> {
    let (write, exec) = prot;
    let hit = |e: &Entry| e.pid == pid && overlap(e.place(), (at, end)).is_some();
    for (i, e) in all.iter().enumerate().filter(|(_, e)| hit(e) && (write || exec)) {
        if exec || !e.may_write {
            return Err(errno::EACCES);
        }
        let becomes = Entry { writer: true, ..e.clone() };
        if all.iter().enumerate().any(|(j, o)| j != i && conflict(o, &becomes)) {
            return Err(errno::ENOSYS);
        }
    }
    Ok(())
}

/// The maps of `pid` in `[at, end)` are writers from here on: a page made
/// read-only again may still hold what was written while it could be.
pub fn writers(all: &mut [Entry], pid: u32, at: u64, end: u64) {
    let hit = |e: &Entry| e.pid == pid && overlap(e.place(), (at, end)).is_some();
    all.iter_mut().filter(|e| hit(e)).for_each(|e| e.writer = true);
}
