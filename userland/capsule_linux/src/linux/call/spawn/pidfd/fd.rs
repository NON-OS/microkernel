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

//! A pidfd in a guest's table: what poll reads on it, what it keeps of its
//! process, and the family's note that keeps that current.

use alloc::vec::Vec;

use super::life::{bits as poll_bits, kept, life};
use crate::linux::guest::{Guest, Kind};

const POLLNVAL: u16 = 0x020;

/// poll, select and epoll's readiness, for net::ready.
pub fn bits(guest: &Guest, fd: u64) -> u16 {
    match guest.fds.get(fd as usize).filter(|f| f.kind == Kind::Pidfd) {
        Some(f) => poll_bits(f.handle, f.ended),
        None => POLLNVAL,
    }
}

/// The pid a pidfd keeps, 0 once its process is gone, and whether the
/// descriptor is O_NONBLOCK; None for a descriptor that is no pidfd.
pub fn child(guest: &Guest, fd: u64) -> Option<(u32, bool)> {
    let f = guest.fds.get(fd as usize).filter(|f| f.kind == Kind::Pidfd)?;
    Some((f.handle, f.nonblock))
}

/// The note taken before each answer, as the pipes' ends are noted: every
/// pidfd anywhere in the family learns whether its process runs, has ended
/// or is gone. A process is reaped only between answers, so no pidfd keeps
/// its pid by the time an answer can fork a process given the same one.
pub fn note(guests: &mut [Guest]) {
    if !guests.iter().any(|g| g.fds.iter().any(|f| f.kind == Kind::Pidfd)) {
        return;
    }
    let live: Vec<u32> = guests.iter().map(|g| g.pid).collect();
    let zombies: Vec<u32> = guests.iter().flat_map(|g| g.ended.iter().map(|e| e.0)).collect();
    let fds = guests.iter_mut().flat_map(|g| g.fds.iter_mut());
    for f in fds.filter(|f| f.kind == Kind::Pidfd) {
        (f.handle, f.ended) = kept(f.handle, life(f.handle, &live, &zombies));
    }
}
