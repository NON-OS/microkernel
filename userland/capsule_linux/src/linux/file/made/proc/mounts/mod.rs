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
 * The mounts a family sees, in one table: /proc/<pid>/mounts, mountinfo,
 * /proc/filesystems, statfs's f_type and stat's st_dev all come from it,
 * so they agree with each other.
 *
 * The tree itself is the store, read-only to a guest; the family's private
 * directories are writable, and are the tmpfs mounts a Linux system has in
 * the same places; /dev, /proc and /sys are made by the personality.
 */

mod files;
mod table;

pub use files::{filesystems, mountinfo, mounts};
pub use table::{dev, of, MOUNTS};
