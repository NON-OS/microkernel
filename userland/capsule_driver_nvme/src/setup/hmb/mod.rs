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

//! Giving the controller what it asks for before any I/O queue exists: one
//! queue pair, and the host memory buffer a DRAM-less drive keeps its tables
//! in. Neither is needed to serve the drive, so neither may cost it: a
//! refusal leaves the drive served without, a command that never completes
//! fails the attempt with the controller reset before the memory goes, and
//! once an attempt that asked for them fails, later attempts do not ask.

mod extras;
mod held;
mod map;
mod offer;
mod queues;

pub use extras::{extras, extras_failed};
pub use held::Hmb;
pub use offer::host_memory;
pub use queues::number_of_queues;
