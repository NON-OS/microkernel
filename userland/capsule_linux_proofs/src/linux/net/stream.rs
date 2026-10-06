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

//! Host double for net.sockets' close (the capsule's net/stream.rs), which
//! the socket table calls when it lets a mixnet stream go. Each close is
//! recorded for the thread that made it, so tests running side by side do
//! not see each other's.

use std::cell::RefCell;

thread_local! {
    static CLOSED: RefCell<Vec<u32>> = const { RefCell::new(Vec::new()) };
}

pub fn close(handle: u32) {
    CLOSED.with(|c| c.borrow_mut().push(handle));
}

/// The handles closed since the last look.
pub fn closed() -> Vec<u32> {
    CLOSED.with(|c| c.borrow_mut().drain(..).collect())
}
