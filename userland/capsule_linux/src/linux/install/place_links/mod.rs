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

//! A package's links, written into the table the personality follows.
//!
//! The store holds files and nothing else, so a link is a `path target` line
//! in /etc/nonos-links, the table busybox's applets already resolve through.
//! A symbolic link keeps its target as written, relative or not; a hard link
//! names another member of the archive, so it becomes an absolute one.
//! Resolution passes every result through `visible`, so no link leaves the
//! guest's tree whatever it says.

mod add;
pub(super) mod table;

pub(super) use add::record;
pub(super) use table::unrecord;
