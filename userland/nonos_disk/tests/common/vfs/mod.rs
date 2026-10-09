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

//! The vfs's own store decoding, `store_header.rs` and `store_toc.rs` from
//! `userland/capsule_vfs/src/blk/`, compiled for the host. Their error type
//! and wire constants are replaced by the one variant and the one size
//! these two files use; the rest of those modules reaches the kernel.

mod error;
mod load;
#[path = "../../../../capsule_vfs/src/blk/store_header.rs"]
mod store_header;
/* The installer's room check, `room_for`, is mounted and not called here. */
#[path = "../../../../capsule_vfs/src/blk/store_toc.rs"]
#[allow(dead_code)]
mod store_toc;
mod wire;

pub use load::load;
