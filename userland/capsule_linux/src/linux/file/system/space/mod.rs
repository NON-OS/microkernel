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
 * How much room a mount has, for statfs, from the family's own files and
 * nothing else: the store's use as a whole would let a guest watch a
 * sibling write, so it is never read.
 *
 * The tree at / is read-only to a guest: its size is the bytes the store
 * holds under the family's root, and none of it is free. The private
 * directories share one quota, declared::PRIVATE bytes and
 * declared::PRIVATE_NAMES names, less what the family keeps there, in the
 * store or still in its cache (quota.rs). /dev, /proc and /sys hold no
 * bytes, which Linux reports as no blocks.
 */

pub mod quota;
mod room;
mod used;

pub use quota::Kept;
pub use room::{of, BSIZE};
pub use used::{room_for, within};
