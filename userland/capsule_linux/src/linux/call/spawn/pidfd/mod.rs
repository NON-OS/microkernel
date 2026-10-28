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

//! pidfds: the descriptor clone's CLONE_PIDFD gives a parent for its child.
//! One refers to one process: poll, select and epoll read it POLLIN once
//! that process has ended, waitid(P_PIDFD) waits for it, and
//! pidfd_send_signal signals it. pidfd_open is not served, so every pidfd
//! here is a clone's.

mod fd;
mod give;
pub mod life;
mod send;

pub use fd::{bits, child, note};
pub use give::{give, reserve};
pub use send::send_signal;
