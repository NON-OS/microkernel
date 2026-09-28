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
 * The family's copy of each file it is writing, one per path.
 *
 * On Linux every descriptor on a file, in every process, reads and writes
 * the same page cache, so a write through one is seen at once through the
 * others and by stat. The store is written whole, so the bytes a family is
 * changing are kept here, once per path, and every descriptor on the path
 * reads and writes this copy: a dup, a fork's copy and a second open all
 * meet the same bytes. The copy goes to the store at close and at fsync.
 */

mod change;
mod flush;
mod table;
mod take;

pub use change::{resize, write};
pub use flush::flush;
pub use table::{held, size};
pub use take::{hold, read};
