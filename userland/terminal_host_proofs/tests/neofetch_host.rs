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
// Host-side harness for the two-column `neofetch` splash: the emblem's column
// width, the zip that lays it beside the info rows, and the colour strip.
//
// Only the pure modules are pulled in. `run.rs` is left out on purpose: it
// borrows `State` and calls `nonos_libc`, neither of which this crate links.
// The modules are included by path so this cannot pass against a copy
// that has drifted from what the capsule runs.

extern crate alloc;

#[path = "neofetch_host/columns.rs"]
mod columns;
#[path = "neofetch_host/logo.rs"]
mod logo;
#[path = "neofetch_host/mounts.rs"]
mod mounts;
#[path = "neofetch_host/padding.rs"]
mod padding;
#[path = "neofetch_host/strips.rs"]
mod strips;
#[path = "neofetch_host/support.rs"]
mod support;

use mounts::neofetch;
