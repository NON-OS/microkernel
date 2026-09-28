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
 * `symlinkat` and `linkat`; the plain forms are these at AT_FDCWD.
 *
 * A symbolic link joins the family's link table, where the image's own links
 * are, and only where the guest may write. A hard link is the same bytes
 * under a second name, copied: the store has no inodes to share, and a copy
 * keeps what programs rely on, that removing the old name leaves the new.
 */

mod calls;
mod free;

pub use calls::{linkat, symlinkat};
