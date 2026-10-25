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

//! The power-off wipe's entry to the paging manager. The other CPUs are
//! parked by then and one may hold the lock, so it is tried once, never
//! waited on: `None` when it is held.

use super::globals::PAGING_MANAGER;

pub fn scrub_every_user_space() -> Option<usize> {
    PAGING_MANAGER.try_lock().map(|manager| manager.scrub_user_spaces())
}
