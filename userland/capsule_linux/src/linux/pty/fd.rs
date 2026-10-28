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

//! Which pair a descriptor is on, what it can do now in poll's bits, and
//! what a read on it would find; and the family's note and signals.

use alloc::vec::Vec;

use crate::linux::guest::{Fd, Guest, Kind};

use super::cell::with;
use super::name::of_handle;
use super::table::To;

const POLLNVAL: u16 = 0x020;

/// The pair a descriptor is on, and whether it is the master.
pub fn side(guest: &Guest, fd: u64) -> Option<(u32, bool)> {
    let f = guest.fds.get(fd as usize).filter(|f| f.kind == Kind::Pty)?;
    Some(of_handle(f.handle))
}

/// poll, select and epoll's readiness, for net::ready.
pub fn bits(guest: &Guest, fd: u64) -> u16 {
    let Some((n, master)) = side(guest, fd) else {
        return POLLNVAL;
    };
    with(|t| t.get(n).map_or(POLLNVAL, |p| p.bits(master)))
}

/// FIONREAD.
pub fn queued(guest: &Guest, fd: u64) -> Option<u64> {
    let (n, master) = side(guest, fd)?;
    with(|t| t.get(n).map(|p| p.queued(master) as u64))
}

/// The note taken before each answer: every pseudo-terminal descriptor
/// open anywhere in the family (`fds`), as the pipes' ends are noted.
pub fn note<'a>(fds: impl Iterator<Item = &'a Fd>) {
    let open: Vec<(u32, bool)> =
        fds.filter(|f| f.kind == Kind::Pty).map(|f| of_handle(f.handle)).collect();
    with(|t| t.note(open.into_iter()));
}

/// The numbers /dev/pts lists.
pub fn names() -> Vec<u32> {
    with(|t| t.names())
}

/// The signals raised since last asked, for the serve loop to deliver.
pub fn signals() -> Vec<(To, u8)> {
    with(|t| core::mem::take(&mut t.signals))
}
