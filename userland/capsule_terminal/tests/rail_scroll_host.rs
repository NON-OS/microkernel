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
// Host-side harness for the rail's scrolled-column geometry: band clipping and
// visibility, the section heights and how the offset is clamped against the
// content. Compiled with the host toolchain:
//   rustc --edition 2021 --test tests/rail_scroll_host.rs -o /tmp/rail_scroll_host && /tmp/rail_scroll_host
//
// Split out of `rail_host.rs`, which covers the navigation rows and hit-test.
// `Rect` and `lh()` are mirrored here as plain data for the same reason.

#[path = "rail_scroll_host/bands.rs"]
mod bands;
#[path = "rail_scroll_host/extent.rs"]
mod extent;
#[path = "rail_scroll_host/mounts.rs"]
mod mounts;
#[path = "rail_scroll_host/sections.rs"]
mod sections;
#[path = "rail_scroll_host/support.rs"]
mod support;

use mounts::{layout, paint};
