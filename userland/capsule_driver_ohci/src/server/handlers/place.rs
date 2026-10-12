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

//! The polled places: a pool of buffers a device holds while its polled
//! bulk IN is armed. Devices on one root port, below a hub, poll side by
//! side, each in its own place; a place is free again once its data is
//! taken or the pipe is reset or given back.

use crate::devices::table::Table;
use crate::sched::layout::POLL_PLACES;

/// A place no device holds, if any.
pub fn free(t: &Table) -> Option<usize> {
    let held = |p: usize| t.devs.iter().flatten().any(|d| d.poll.is_some_and(|(q, _)| q == p));
    (0..POLL_PLACES).find(|&p| !held(p))
}
