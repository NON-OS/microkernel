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

//! Where a memfd landed, and what was staged on it.

use alloc::vec::Vec;

use crate::linux::guest::{Guest, Kind};

/// Content this capsule put on a descriptor before the guest mapped it.
pub fn staged(guest: &Guest, fd: u64) -> Option<Vec<u8>> {
    match guest.fds.get(fd as usize) {
        Some(e) if e.kind == Kind::Memfd && !e.pending.is_empty() => Some(e.pending.clone()),
        _ => None,
    }
}

/// Where a mapped memfd lives in the guest, which is what a shm pool
/// needs to find its pixels.
pub fn mapped_at(guest: &Guest, fd: u64) -> Option<(u64, u64)> {
    match guest.fds.get(fd as usize) {
        Some(e) if e.kind == Kind::Memfd && e.offset != 0 => Some((e.offset, e.size)),
        _ => None,
    }
}

/// Record the address a mapping landed on. The offset field carries it,
/// because a memfd has no read position for it to mean anything else.
pub fn set_mapped(guest: &mut Guest, fd: u64, at: u64) {
    if let Some(entry) = guest.fds.get_mut(fd as usize) {
        if entry.kind == Kind::Memfd {
            entry.offset = at;
        }
    }
}

/// The memfd identity behind `fd`, which its dups share (fd_make.rs).
pub fn memfd_id(guest: &Guest, fd: u64) -> Option<u32> {
    match guest.fds.get(fd as usize) {
        Some(e) if e.kind == Kind::Memfd => Some(e.handle),
        _ => None,
    }
}

/// Where memfd `id` is mapped now. A dup keeps the address its original had
/// when it was made, so a client that maps again after passing the dup (as
/// libwayland-cursor does when it grows a pool before the request leaves) is
/// found through whichever descriptor saw the new mapping.
pub fn mapped_now(guest: &Guest, id: u32) -> Option<(u64, u64)> {
    let live = |e: &&crate::linux::guest::Fd| {
        e.kind == Kind::Memfd && e.handle == id && e.offset != 0 && guest.mapped_from(e.offset) > 0
    };
    guest.fds.iter().filter(live).last().map(|e| (e.offset, e.size))
}
