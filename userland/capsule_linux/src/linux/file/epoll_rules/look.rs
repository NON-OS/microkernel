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

//! epoll's event bits, and what one look at an entry reports. Pure, so the
//! host proofs hold it.

use crate::linux::guest::EPOLLET;

pub const EPOLL_CTL_ADD: u64 = 1;
pub const EPOLL_CTL_MOD: u64 = 3;
pub(super) const EPOLLIN: u32 = 0x001;
pub(super) const EPOLLOUT: u32 = 0x004;
pub(super) const EPOLLERR: u32 = 0x008;
pub(super) const EPOLLHUP: u32 = 0x010;
pub(super) const EPOLLEXCLUSIVE: u32 = 1 << 28;
pub(super) const EPOLLWAKEUP: u32 = 1 << 29;

/// What an entry sees of its descriptor's readiness `level`: what it asked
/// for, and hang-up and error, which are reported whether asked for or not.
pub fn seen(level: u32, events: u32) -> u32 {
    level & (events | EPOLLHUP | EPOLLERR)
}

/// What an entry reports: what it sees, and under EPOLLET only what rose
/// since `fired`, what it saw at the last look.
pub fn live(level: u32, events: u32, fired: u32) -> u32 {
    match events & EPOLLET {
        0 => seen(level, events),
        _ => seen(level, events) & !fired,
    }
}
