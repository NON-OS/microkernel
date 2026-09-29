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

//! `poll` over the guest's descriptors.

use crate::linux::guest::{Guest, Kind};

use super::poll_socket::socket_bits;

const POLLIN: u16 = 0x001;
const POLLOUT: u16 = 0x004;
const POLLNVAL: u16 = 0x020;
/// Reported whether asked for or not, by poll and by epoll alike.
pub const POLLERR: u16 = 0x008;
pub const POLLHUP: u16 = 0x010;

/// What `fd` can do right now, in poll's bits.
pub fn ready(guest: &Guest, fd: u64) -> u16 {
    match guest.fds.get(fd as usize).map(|f| &f.kind) {
        Some(Kind::Free) | None => POLLNVAL,
        Some(Kind::Socket) => match guest.socket_handle(fd) {
            Some(handle) => socket_bits(handle),
            None => POLLNVAL,
        },
        Some(Kind::Timer) => crate::linux::file::timer_bits(guest, fd),
        Some(Kind::Pipe) => crate::linux::call::pipe_bits(guest, fd),
        Some(Kind::Event) => crate::linux::file::event_bits(guest, fd),
        Some(Kind::Signal) => crate::linux::call::signalfd_bits(guest, fd),
        Some(Kind::Resolver) => resolver_bits(guest, fd),
        Some(_) => POLLIN | POLLOUT,
    }
}

/// Readable once an answer is waiting, and always writable: a query is taken
/// whenever it is offered.
fn resolver_bits(guest: &Guest, fd: u64) -> u16 {
    match guest.fds.get(fd as usize) {
        Some(e) if !e.replies.is_empty() => POLLIN | POLLOUT,
        _ => POLLOUT,
    }
}
