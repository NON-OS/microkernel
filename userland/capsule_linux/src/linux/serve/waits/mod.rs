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

//! Calls that wait for a descriptor: `epoll_wait`, `poll`, `ppoll`,
//! `select` and `pselect6` until their timeout, and a read or write that
//! would block (`waits_try`).
//!
//! Each is tried when it arrives. One that cannot complete is left parked
//! in its trap, and the family tries it again after every answer and at its
//! deadline (`family_waits`), so the other threads and processes it hosts
//! keep being served while it waits.

mod io;
mod timed;

pub use io::io;
pub use timed::timed;
