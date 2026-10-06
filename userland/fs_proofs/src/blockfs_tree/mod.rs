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

//! The kernel's file index trees, assembled for the host.
//!
//! These are the shipping kernel modules, included by `#[path]`. They touch
//! no device and no cipher: blocks come from a store the caller supplies, so
//! here an in-memory one stands in for the sealed disk and every boundary of
//! the index is written and read back exactly as ring 0 does it.

#[path = "../../../../src/fs/blockfs/file_cache.rs"]
pub mod file_cache;
#[path = "../../../../src/fs/blockfs/file_consts.rs"]
pub mod file_consts;
#[path = "../../../../src/fs/blockfs/index_block.rs"]
pub mod index_block;
#[path = "../../../../src/fs/blockfs/tree_ptrs.rs"]
pub mod tree_ptrs;
#[path = "../../../../src/fs/blockfs/tree_range.rs"]
pub mod tree_range;
#[path = "../../../../src/fs/blockfs/tree_reader.rs"]
pub mod tree_reader;
#[path = "../../../../src/fs/blockfs/tree_shape.rs"]
pub mod tree_shape;
#[path = "../../../../src/fs/blockfs/tree_store.rs"]
pub mod tree_store;
#[path = "../../../../src/fs/blockfs/tree_writer.rs"]
pub mod tree_writer;
#[path = "../../../../src/fs/blockfs/tree_writer_end.rs"]
pub mod tree_writer_end;

// The byte helpers the directory layout uses too: one copy of each kernel
// file, mounted at the crate root and reached by the name the trees use.
pub(crate) use crate::{read_u64, write_u32, write_u64};

mod before;
mod cached;
mod deep_file;
mod disk;
mod mem;
mod numbered_file;
mod packed;
mod packed_run;
mod run;
mod sector_open;
mod tests;
mod tests_cache;
mod tests_cache_id;
mod tests_cache_stale;
mod tests_capacity;
mod tests_deep;
mod tests_edges;
mod tests_index;
mod tests_layout;
mod tests_model_size;
mod tests_reads;
