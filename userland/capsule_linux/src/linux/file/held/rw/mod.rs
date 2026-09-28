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
 * The bytes of a file descriptor at an offset, in and out. read, write,
 * the p- and v- forms, sendfile and copy_file_range all come here, so a
 * file reads the same whichever call asks.
 *
 * In order: the family's copy of a file it is writing; the store, through
 * the descriptor's stream, opened again for a descriptor that dup or fork
 * made without one.
 */

mod read;
mod write;

pub use read::{read_at, MAX_IO};
pub use write::write_at;
