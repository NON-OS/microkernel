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

//! The pseudo-terminals' pure rules: the line discipline, the pair and the
//! table of pairs, each mounted as it ships, and the tests that hold them.

#[path = "../../../../capsule_linux/src/linux/pty/disc.rs"]
pub mod disc;
#[path = "../../../../capsule_linux/src/linux/pty/edit.rs"]
pub mod edit;
#[path = "../../../../capsule_linux/src/linux/pty/flags.rs"]
pub mod flags;
#[path = "../../../../capsule_linux/src/linux/pty/input.rs"]
pub mod input;
#[path = "../../../../capsule_linux/src/linux/pty/lookup.rs"]
pub mod lookup;
#[path = "../../../../capsule_linux/src/linux/pty/name.rs"]
pub mod name;
#[path = "../../../../capsule_linux/src/linux/pty/note.rs"]
pub mod note;
#[path = "../../../../capsule_linux/src/linux/pty/out.rs"]
pub mod out;
#[path = "../../../../capsule_linux/src/linux/pty/pair.rs"]
pub mod pair;
#[path = "../../../../capsule_linux/src/linux/pty/pair_io.rs"]
pub mod pair_io;
#[path = "../../../../capsule_linux/src/linux/pty/side.rs"]
pub mod side;
#[path = "../../../../capsule_linux/src/linux/pty/side_poll.rs"]
pub mod side_poll;
#[path = "../../../../capsule_linux/src/linux/pty/table.rs"]
pub mod table;
#[path = "../../../../capsule_linux/src/linux/pty/table_io.rs"]
pub mod table_io;

#[cfg(test)]
mod discipline_tests;
#[cfg(test)]
mod hangup_tests;
#[cfg(test)]
mod naming_tests;
#[cfg(test)]
mod raw_tests;
#[cfg(test)]
mod signal_tests;
#[cfg(test)]
mod support;
