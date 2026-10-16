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
// Host-side harness for the rail's navigation rows and their hit-test.
// Compiled with the host toolchain:
//   rustc --edition 2021 --test tests/rail_host.rs -o /tmp/rail_host && /tmp/rail_host
//
// `Rect` comes from `nonos_toolkit` and `lh()` from the TTF metrics, neither of
// which resolves outside cargo, so both are mirrored here as plain data. The
// arithmetic under test lives in the real `rail_band.rs`, `rail_row.rs` and
// `rail_left_geom.rs`, which are pulled in by path. The scrolled-column
// geometry is covered by `rail_scroll_host.rs`, the formatters by
// `rail_fmt_host.rs`.

#[path = "rail_host/disk.rs"]
mod disk;
#[path = "rail_host/hits.rs"]
mod hits;
#[path = "rail_host/memory.rs"]
mod memory;
#[path = "rail_host/mounts.rs"]
mod mounts;
#[path = "rail_host/mounts_paint.rs"]
mod mounts_paint;
#[path = "rail_host/net.rs"]
mod net;
#[path = "rail_host/rows.rs"]
mod rows;
#[path = "rail_host/support.rs"]
mod support;
#[path = "rail_host/unsupported.rs"]
mod unsupported;

use mounts::{layout, rail, term};
use mounts_paint::paint;
