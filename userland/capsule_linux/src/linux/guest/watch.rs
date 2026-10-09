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

//! One entry of an epoll interest list.

/// Report a readiness once as it rises, not for as long as it holds.
pub const EPOLLET: u32 = 1 << 31;
/// Report once, then nothing until the entry is modified.
pub const EPOLLONESHOT: u32 = 1 << 30;

#[derive(Clone, Copy)]
pub struct Watch {
    pub fd: u64,
    /// What the program asked for, EPOLLET and EPOLLONESHOT included.
    pub events: u32,
    /// The token the program gets back, its own and never interpreted.
    pub data: u64,
    /// The readiness seen at the last look. Under EPOLLET only what was not
    /// already in it is reported.
    pub fired: u32,
    /// Cleared once an EPOLLONESHOT entry has reported.
    pub armed: bool,
}

impl Watch {
    pub fn new(fd: u64, events: u32, data: u64) -> Watch {
        Watch { fd, events, data, fired: 0, armed: true }
    }
}
