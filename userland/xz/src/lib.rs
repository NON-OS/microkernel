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

//! xz decoding: the .xz container (streams, blocks, index, padding) around
//! LZMA2, with every check type xz writes. Decode only; the one filter read
//! is LZMA2, and a block that names another is refused, never skipped.
//!
//! The output total is checked after each chunk is appended, so it can pass
//! `MAX_OUT` by one chunk before the call fails. Every failure is `None`.

#![no_std]

extern crate alloc;

mod block;
mod block_header;
mod check;
mod crc32;
mod crc64;
mod index;
mod limits;
mod lzma2;
mod lzma2_chunk;
mod lzma_copy;
mod lzma_decode;
mod lzma_dist;
mod lzma_len;
mod lzma_literal;
mod lzma_model;
mod lzma_model_new;
mod lzma_rep;
mod range;
mod range_tree;
mod stream;
mod varint;

pub use limits::MAX_OUT;
pub use stream::decompress;
