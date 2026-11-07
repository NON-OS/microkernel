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

//! The kernel's block reclaim (src/fs/blockfs/reclaim), walked over volumes
//! the kernel's own index, tree and directory-record code laid out.
//!
//! The walk and the run arithmetic are mounted as they ship, beside the
//! layout files they read, so what is judged reached here is what ring 0
//! judges reached.

#[path = "../../../../src/fs/blockfs/dir_block.rs"]
pub mod dir_block;
#[path = "../../../../src/fs/blockfs/dir_block_header.rs"]
pub mod dir_block_header;
#[path = "../../../../src/fs/blockfs/dir_consts.rs"]
pub mod dir_consts;
#[path = "../../../../src/fs/blockfs/dir_name_match.rs"]
pub mod dir_name_match;
#[path = "../../../../src/fs/blockfs/file_consts.rs"]
pub mod file_consts;
#[path = "../../../../src/fs/blockfs/index_block.rs"]
pub mod index_block;
#[path = "../../../../src/fs/blockfs/read_u32.rs"]
pub mod read_u32;
#[path = "../../../../src/fs/blockfs/tree_ptrs.rs"]
pub mod tree_ptrs;
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
pub(crate) use crate::{read_u64, write_u32, write_u64};

pub mod reclaim;

#[cfg(test)]
mod tests;
mod vol;
