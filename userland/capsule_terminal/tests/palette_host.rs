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
// Host-side harness for the command palette's matcher, query buffer and panel
// geometry. Compiled with the host toolchain:
//   rustc --edition 2021 --test tests/palette_host.rs -o /tmp/h && /tmp/h
//
// `Rect` comes from `nonos_toolkit`, which does not resolve outside cargo, so
// it is mirrored here as plain data. `index.rs` is left out on purpose: it
// borrows `State`, which drags in the whole capsule.

#[path = "palette_host/filtering.rs"]
mod filtering;
#[path = "palette_host/geometry.rs"]
mod geometry;
#[path = "palette_host/mounts.rs"]
mod mounts;
#[path = "palette_host/state.rs"]
mod state;
#[path = "palette_host/support.rs"]
mod support;
#[path = "palette_host/verb_words.rs"]
mod verb_words;

use mounts::{layout, palette};
