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

//! A FAT32 volume laid out in one pass from a fixed tree.
//!
//! Firmware boots from FAT and nothing else, so this is the filesystem the
//! ESP has to be. It is written, never mutated: the tree is known before the
//! first sector goes out, every directory and file sits in one contiguous
//! run of clusters, the FAT is a straight chain per run, and the free count
//! in FSInfo is exact. There is no allocator because nothing is allocated
//! after the fact. The session module turns this layout into writes.

mod bpb;
mod dir;
mod fat;
mod geometry;
mod tree;
mod write;

pub use dir::encode as encode_dir;
pub use fat::build as build_fat;
pub use geometry::{plan, Geometry, PlanError, FAT_COUNT, RESERVED_SECTORS};
pub use tree::{place_with, Content, Node, Placed};
pub use write::{build_reserved, data_lba, WriteVolumeError};
