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
 * Following a path through the links in it: the image's own, and the ones
 * /dev and /proc make (/proc/self, /dev/fd, /proc/<pid>/cwd, /root, /exe).
 *
 * The path is walked a name at a time, as Linux walks it: a link is
 * followed where it stands, and a `..` after it goes to the parent of
 * where the link led, not of the link. `..` at the root stays at the root,
 * so every result is a path of the family's own tree, and /proc/<pid>/root
 * is that root. A descriptor's link that names no path, a pipe or a
 * socket, is not walked through, as Linux cannot walk through it either.
 */

mod link;
mod path;
mod state;
mod step;

pub use path::walk;
pub use step::{follow, Step};
