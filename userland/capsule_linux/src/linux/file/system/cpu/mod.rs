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

/*
 * What the kernel measured of the family's own threads: CPU ticks split
 * into user and kernel, page faults, context switches and resident pages.
 *
 * The kernel shows a supervisor these for the guests it hosts and for no
 * one else's, so every figure is one of the family's own. Only the rows
 * for the pids asked about are kept; the rest of the machine's table is
 * read past and dropped, and nothing of it reaches a guest.
 */

mod ended;
mod usage;

pub use ended::threads_of;
pub use usage::{usage, Usage};
