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

//! Host double for net.anon's close (the capsule's net/anon_stream.rs),
//! which the socket table calls on close, on shutdown of both sides, and
//! when an entry is freed. Each OP_CLOSE_STREAM it would send is recorded
//! for the thread that made it.

use std::cell::RefCell;

use super::sock::Close;

thread_local! {
    static CLOSED: RefCell<Vec<Close>> = const { RefCell::new(Vec::new()) };
}

pub fn close(c: Close) {
    CLOSED.with(|v| v.borrow_mut().push(c));
}

/// The streams closed since the last look.
pub fn closed() -> Vec<Close> {
    CLOSED.with(|v| v.borrow_mut().drain(..).collect())
}
