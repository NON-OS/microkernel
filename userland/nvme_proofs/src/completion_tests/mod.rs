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

//! The admin and I/O completion waits share one loop. Every completion entry
//! is written by the controller, so the proofs run that loop over scripted
//! rings: an entry is taken only when its phase tag is this pass's and it
//! names the command in flight, any status bit fails the command, the SQ
//! head the controller reports never moves the driver's cursor, every slot
//! read and every doorbell value stays inside the ring, and the wait ends
//! once its deadline passes.

mod deadline_tests;
mod flood_tests;
mod late_tests;
mod match_tests;
mod phase_tests;
mod ring;
mod status_tests;
mod torn_tests;
mod torn_wait_tests;
mod wait;

pub(crate) use ring::{entry, ok_status, RINGS};
pub(crate) use wait::wait_on;
