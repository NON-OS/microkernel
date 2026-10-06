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

//! Network work the window's own thread can carry without stopping.
//!
//! The refresh used to run whole inside one tick: through the Nym mixnet a
//! routed read may wait a minute and a half, and the window froze for all
//! of it. A second thread would have to make the calls instead, and the
//! kernel does not let one: a thread from MkThreadSpawn has no reply inbox
//! of its own, so every mk_ipc_call it makes fails, and it inherits only
//! the ambient capabilities, without Network, so it may not reach net.dns,
//! net.sockets, net.socks5 or net.anon at all. So the work is cut instead:
//! a `Job` is stepped once a tick, and no step waits on the network longer
//! than `SLICE_MS`. Only the finished result is applied to the state.

mod exchange;
pub mod gather;
mod io;
mod job;
mod open;
mod probe;

pub use exchange::{Ended, Exchange};
pub use job::{Finished, Followed, Job};

/// The longest one step waits on the network.
pub const SLICE_MS: u64 = 50;
