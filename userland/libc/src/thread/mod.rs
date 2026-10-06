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

//! Threads of this process, for no_std capsules.
//!
//! A thread spawned with MkThreadSpawn shares the process's address space,
//! carries its capabilities and has its own reply inbox, `proc.<tid>`, so it
//! can make the slow IPC call a window thread would otherwise sit in, and
//! hand the answer back through memory both can see. See `seat.rs` for what a
//! thread is to the services it calls.

mod entry;
mod handoff;
mod seat;
mod spawn;

pub use handoff::{Handoff, Look};
pub use seat::{WorkerSeat, WORKER_STACK};
pub use spawn::mk_thread_spawn;
