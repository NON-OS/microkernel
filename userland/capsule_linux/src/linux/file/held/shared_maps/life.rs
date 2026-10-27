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

//! A shared map made, and what mprotect may do with one.

use super::super::super::{models, synth};
use super::super::desc;
use super::asks::{protects, refused, writers};
use super::entry::Entry;
use super::rules::admits;
use super::table::with;
use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

/// mmap's and mprotect's bits, as Linux numbers them.
const PROT_WRITE: u64 = 0x2;
const PROT_EXEC: u64 = 0x4;

/// The map `fd` would make at `[at, at + span)` from `off`, if mmap may
/// make it (asks.rs).
pub fn asked(g: &Guest, fd: u64, at: u64, span: u64, off: u64, prot: u64) -> Result<Entry, i64> {
    let f = g.fds.get(fd as usize).filter(|f| f.is_open()).ok_or(errno::EBADF)?;
    let stored = f.kind == Kind::File && !synth::owns(&f.path) && !models::owns(&f.path);
    let (write, exec) = (prot & PROT_WRITE != 0, prot & PROT_EXEC != 0);
    if let Some(e) = refused(stored, desc::reads(f), f.writable, write, exec) {
        return Err(e);
    }
    /* Linux's EOVERFLOW, a file range past the largest offset, as EINVAL. */
    off.checked_add(span).ok_or(errno::EINVAL)?;
    let (path, may_write) = (f.path.clone(), f.writable);
    Ok(Entry { pid: g.pid, at, span, path, off, writer: write, may_write, orphan: false })
}

/// Whether `new` may join the table; `fixed` for MAP_FIXED (rules.rs).
pub fn admit(new: &Entry, fixed: bool) -> Result<(), i64> {
    with(|all| admits(all, new, fixed))
}

/// `new`, laid down in its process: from here it is shown every write of
/// its file, and what it changes goes back.
pub fn record(new: Entry) {
    with(|all| all.push(new));
}

/// mprotect of `[at, at + span)` of `pid` to write and exec as `prot`
/// asks (asks.rs). A map given write is a writer from here on.
pub fn protect(pid: u32, at: u64, span: u64, prot: u64) -> Result<(), i64> {
    let (end, write) = (at.saturating_add(span), prot & PROT_WRITE != 0);
    with(|all| {
        protects(all, pid, at, end, (write, prot & PROT_EXEC != 0))?;
        if write {
            writers(all, pid, at, end);
        }
        Ok(())
    })
}
