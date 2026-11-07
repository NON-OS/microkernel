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


//! The person's folders (Documents, Downloads, Music, Pictures, Videos),
//! through the user-files broker, for
//! a program whose policy grants FILES_USER.
//!
//! A broker file is a `Kind::File` descriptor like any other: dup, fork,
//! close-on-exec, offsets and locks are the personality's own rules. Only
//! its bytes, its metadata and its names go to the broker, each call at the
//! point the store would have been asked (open/named.rs, held/rw, meta,
//! dirops, rename, size, fsync). Nothing of it passes through the store or
//! the family's write cache, so a file is never held whole in this process.

mod flags;
mod held;
mod io;
mod link;
mod list;
mod meta;
mod names;
mod open;
mod place;
pub mod place_rule;
mod size;

pub use held::gone;
pub use io::{is_user_file, owns_fd, read, write};
pub use list::home_names;
pub use meta::{look, stat};
pub use names::{chmod, mkdir, no_link, rename, stamp, unlink};
pub use open::open;
pub use size::{fsync, ftruncate, truncate};
