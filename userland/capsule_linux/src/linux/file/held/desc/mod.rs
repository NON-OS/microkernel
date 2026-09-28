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
 * The open file description behind a descriptor on a file or directory.
 *
 * A store file has no server handle number of its own, so its `handle`
 * holds what belongs to the description rather than to the descriptor: a
 * number naming the description, which dup and fork copy with the rest,
 * whether it was opened O_APPEND, and whether it was opened to read. The
 * description's offset is kept here too: after a fork, a child's write
 * moves the parent's offset, which is how a shell's output and its
 * commands' output land one after the other in the same file. Two descriptors share a description
 * exactly when a dup or a fork made one from the other, as on Linux.
 */

mod handle;
mod offset;
mod shared;

pub use handle::{appends, fresh, of, reads};
pub use offset::{gone, pos, set_pos};
pub use shared::held_elsewhere;
