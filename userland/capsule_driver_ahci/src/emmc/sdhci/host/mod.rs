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

//! The host controller engine: reset, power, clock, bus width and timing,
//! and one command at a time, polled to its end. Every wait is bounded by
//! the clock. A command that fails leaves the CMD and DAT lines reset, so
//! the next command starts on a clean host.

mod cmd;
mod detect;
mod engine;
mod init;
mod issue;
mod limits;
mod power_up;
mod program;
mod reset;
mod say_caps;
mod send;
mod set_clock;
mod snapshot;
mod transfer;
mod wait;
mod width;

pub use cmd::{Cmd, Data};
pub use engine::Host;
pub use limits::SETTLE_MS;
pub use snapshot::Snapshot;
