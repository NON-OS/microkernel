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

//! The calls a network capsule makes into nonos_userland_libc, answered on the
//! host.
//!
//! A capsule's IPC calls go to whatever responder the proof installed, which
//! plays the service on the other end. Its replies to its own callers are
//! kept, in order, for the proof to read. Time stands still unless the proof
//! moves it or the capsule yields, and a yield moves it one millisecond, so a
//! loop that waits on the clock always ends.
//!
//! The state here is process wide, as the capsule's own statics are, so a
//! proof holds `serial()` for its whole run.

mod ipc;
mod process;
mod random;
mod serial;
mod time;

pub use ipc::{mk_ipc_call, mk_ipc_call_timeout, mk_ipc_recv_from, mk_ipc_reply};
pub use ipc::{mk_service_lookup, set_responder, take_replies, Reply};
pub use process::{end_pid, heap_init, mk_debug, mk_exit, mk_pid_alive, revive_all};
pub use random::crypto_random;
pub use serial::serial;
pub use time::{advance, mk_time_adjust, mk_time_millis, mk_uptime_ms, mk_yield, set_time};
