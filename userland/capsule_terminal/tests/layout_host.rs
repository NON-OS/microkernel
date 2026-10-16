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
// Host-side harness for the window band/rail solver. Compiled with the host
// toolchain:
//   rustc --edition 2021 --test tests/layout_host.rs -o /tmp/layout_host && /tmp/layout_host
//
// `src/layout/types.rs` re-exports `Rect` from `nonos_toolkit`, which does not
// resolve outside cargo, so `types` is mirrored here as plain data. The
// arithmetic under test lives entirely in the real `compute.rs` and
// `limits.rs`, which are pulled in by path.

#[path = "layout_host/bands.rs"]
mod bands;
#[path = "layout_host/limits.rs"]
mod limits;
#[path = "layout_host/mounts.rs"]
mod mounts;
#[path = "layout_host/narrow.rs"]
mod narrow;
#[path = "layout_host/rails.rs"]
mod rails;
#[path = "layout_host/row_math.rs"]
mod row_math;
#[path = "layout_host/support.rs"]
mod support;

use mounts::{layout, rows};
