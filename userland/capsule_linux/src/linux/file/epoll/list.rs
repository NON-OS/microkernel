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

//! An epoll list made, an epoll_event read, and a closed descriptor
//! dropped from every list.

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest, Kind};

use super::super::slot::install;

/// `struct epoll_event` is packed on x86_64: a u32 of events then a u64
/// of caller data, twelve bytes and not sixteen.
pub const EVENT_LEN: usize = 12;

pub fn epoll_create(guest: &mut Guest) -> u64 {
    match install(guest, Fd::empty(Kind::Epoll)) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    }
}

/// Drop `fd` from every interest list, as Linux does when a descriptor is
/// closed, so a later descriptor given its number starts unregistered.
pub fn forget(guest: &mut Guest, fd: u64) {
    for list in guest.fds.iter_mut().filter(|f| f.kind == Kind::Epoll) {
        list.watch.retain(|w| w.fd != fd);
    }
}

pub(super) fn read_event(guest: &Guest, at: u64) -> Option<(u32, u64)> {
    let raw = guest.read(at, EVENT_LEN)?;
    let events = u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]);
    let mut data = [0u8; 8];
    data.copy_from_slice(&raw[4..12]);
    Some((events, u64::from_le_bytes(data)))
}
