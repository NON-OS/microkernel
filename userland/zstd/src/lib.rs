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

//! Zstandard decoding, RFC 8878: every block type, Huffman and FSE, repeat
//! offsets and the content checksum. Decode only; no dictionaries.
//!
//! The output total is checked after each block is appended, so it can pass
//! `MAX_OUT` by one block before the call fails. Every failure is `None`: a
//! hostile frame yields no output, never a panic.

#![no_std]

extern crate alloc;

mod back;
mod block;
mod context;
mod frame;
mod frame_header;
mod fse_build;
mod fse_cell;
mod fse_default;
mod fse_read;
mod fwd;
mod huff_build;
mod huff_stream;
mod huff_weights;
mod limits;
mod literals;
mod seq_codes;
mod seq_header;
mod seq_rep;
mod seq_run;
mod xxh64;

pub use frame::decompress;
pub use limits::MAX_OUT;
pub use xxh64::xxh64;
