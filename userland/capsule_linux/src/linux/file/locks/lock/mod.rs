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
 * The family's file locks, with Linux's rules for who conflicts with whom.
 *
 * Three kinds, as on Linux. A flock lock belongs to an open file
 * description, so a dup or a fork shares it, and it goes when the last
 * descriptor on that description closes. A POSIX record lock (F_SETLK)
 * belongs to a process, and goes when that process closes any descriptor
 * on the file, or exits. An OFD record lock (F_OFD_SETLK) is a record lock
 * owned by a description. flock locks never meet record locks; POSIX and
 * OFD locks meet each other. A process's own POSIX locks never conflict
 * with each other: a new one replaces the old over the range it covers.
 */

mod apply;
mod rules;
mod table;

pub use apply::{apply, drop_where};
pub use rules::{blocker, take};
pub use table::{Lock, Owner};
