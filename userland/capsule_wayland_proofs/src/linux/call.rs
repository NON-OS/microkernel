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

//! Clocks and pipes. The clocks are the host's, or a record's (record.rs),
//! so a played-back session sees the times its live one did. The bridge's
//! own pipes are the desktop clipboard's (clip/desk_read.rs), which the host
//! desktop leaves off, so none is made.

use std::cell::Cell;
use std::sync::OnceLock;
use std::time::Instant;

use super::guest::{Fd, Guest, Held};

fn start() -> Instant {
    static START: OnceLock<Instant> = OnceLock::new();
    *START.get_or_init(Instant::now)
}

thread_local! {
    static PLAYED: Cell<Option<u64>> = const { Cell::new(None) };
}

/// The clock set to `ms` for this thread, as a record played back says.
pub fn set_clock(ms: u64) {
    PLAYED.with(|c| c.set(Some(ms)));
}

/// Milliseconds since the family started.
pub fn family_ms() -> u64 {
    PLAYED.with(Cell::get).unwrap_or_else(|| start().elapsed().as_millis() as u64 + 1)
}

/// A moment the desktop double reports is already on the family's clock.
pub fn family_ns(ns: u64) -> u64 {
    ns
}

pub fn now_ms(_clock: u64) -> Option<u64> {
    Some(family_ms())
}

/// The guest pipe `fd` is an end of, and whether it is the write end.
pub fn pipe_end_of(guest: &Guest, fd: u64) -> Option<(usize, bool)> {
    match guest.fds.get(fd as usize)?.held {
        Held::Pipe { slot, write, .. } => Some((slot, write)),
        _ => None,
    }
}

/// Whether the other end of pipe `slot` is open: from the read end, whether
/// its writer has not yet closed.
pub fn pipe_other_end_open(guest: &Guest, slot: usize, writable: bool) -> bool {
    writable || !guest.pipe_eof.get(slot).copied().unwrap_or(true)
}

/// A pipe of the host's, both ends in the guest's table: its slot, the read
/// end and the write end. The read end does not block.
pub fn pipe_pair(guest: &mut Guest, _flags: u64) -> Option<(usize, u64, u64)> {
    let mut ends = [0i32; 2];
    // SAFETY: a new pipe; the read end set not to block.
    unsafe {
        if libc::pipe2(ends.as_mut_ptr(), libc::O_CLOEXEC | libc::O_NONBLOCK) != 0 {
            return None;
        }
    }
    let slot = new_slot(guest);
    let read = Fd::pipe(slot, false, ends[0], true);
    let write = Fd::pipe(slot, true, ends[1], true);
    let read = super::file::install(guest, read)?;
    let write = super::file::install(guest, write)?;
    Some((slot, read, write))
}

/// A pipe slot of its own, empty and open.
pub fn new_slot(guest: &mut Guest) -> usize {
    guest.pipes.push(Vec::new());
    guest.pipe_eof.push(false);
    guest.pipes.len() - 1
}

pub fn close(guest: &mut Guest, fd: u64) -> u64 {
    if let Some(f) = guest.fds.get_mut(fd as usize) {
        f.close();
    }
    0
}
