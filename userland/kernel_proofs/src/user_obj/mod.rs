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
 * The person's files' objects: a path made only from an object number,
 * and the MkData names that can never reach them. The kernel's own files.
 */

#[path = "../../../../src/syscall/microkernel/data/name_shape.rs"]
pub mod name_shape;
#[path = "../../../../src/fs/blockfs_volume/user_obj/path.rs"]
pub mod path;
#[cfg(test)]
mod tests;
