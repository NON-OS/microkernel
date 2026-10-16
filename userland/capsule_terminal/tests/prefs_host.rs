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
// Host-side harness for the on-disk preferences record. Compiled with the
// host toolchain:
//   rustc --edition 2021 --test tests/prefs_host.rs -o /tmp/prefs_host && /tmp/prefs_host
//
// `codec.rs` reaches for `crate::term::dimensions`. In a `--test` binary the
// crate root is this file, so the module tree in `prefs_host/mounts.rs` is
// laid out to match the shipping one and re-exported here, and the `crate::`
// path resolves against it. The font-scale bounds mirror
// `src/term/dimensions.rs`.

#[path = "prefs_host/clamping.rs"]
mod clamping;
#[path = "prefs_host/fallback.rs"]
mod fallback;
#[path = "prefs_host/mounts.rs"]
mod mounts;
#[path = "prefs_host/projects.rs"]
mod projects;
#[path = "prefs_host/shape.rs"]
mod shape;
#[path = "prefs_host/support.rs"]
mod support;
#[path = "prefs_host/versions.rs"]
mod versions;

use mounts::term;
