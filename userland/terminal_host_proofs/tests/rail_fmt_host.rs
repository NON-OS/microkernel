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
//
// Host-side harness for the rail's formatters: the em-dash rendering of a
// metric with no source, and the address writers.
//
// Split out of `rail_host.rs`, which covers the scrolled-column geometry.

#[path = "rail_fmt_host/addresses.rs"]
mod addresses;
#[path = "rail_fmt_host/dashes.rs"]
mod dashes;
#[path = "rail_fmt_host/load.rs"]
mod load;
#[path = "rail_fmt_host/memory_row.rs"]
mod memory_row;
#[path = "rail_fmt_host/mounts.rs"]
mod mounts;
#[path = "rail_fmt_host/sample_rows.rs"]
mod sample_rows;

use mounts::{paint, rail, term};
