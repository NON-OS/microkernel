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
 * Every result is a path of the family's own tree. /proc/<pid>/root is the
 * family's root, so nothing reached through it, or through `..` after it,
 * is outside that root; and a descriptor's link that names no path, a pipe
 * or a socket, is not followed through, as Linux cannot follow it either.
 */

mod path;
mod step;

pub use step::follow;
