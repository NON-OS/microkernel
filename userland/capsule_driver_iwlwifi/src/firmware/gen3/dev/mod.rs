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

//! The running transport: registers, the control and receive-buffer regions,
//! and both queues. It takes received packets in order, hands each to the
//! caller's handler, and runs a host command to its reply. Every wait is one
//! bounded clock wait, and ends early on a firmware or hardware error cause.

mod command;
mod drain;
mod kinds;
mod state;

pub use kinds::{Flow, WaitError, COMMAND_MS};
pub use state::Dev;
