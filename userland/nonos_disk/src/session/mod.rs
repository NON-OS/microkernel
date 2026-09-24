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

//! An install that advances in steps, so a screen can stay live while a
//! hundred megabytes go to a disk.
//!
//! The whole disk is planned before the first sector is written: layout,
//! geometry, placement, the tables, and a queue of jobs in the order they
//! land. Each `step` writes at most a budget of bytes from the front of the
//! queue and reports how far it got. The caller decides the budget from how
//! often it wants to paint; the disk does not care.

mod finish;
mod job;
mod plan;
mod progress;
mod queue;
mod runs;
mod step;
mod verify;

pub use plan::Plan;
pub use progress::Progress;
pub use step::Session;
pub use verify::Verifier;
