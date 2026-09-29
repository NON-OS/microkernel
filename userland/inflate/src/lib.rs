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

#![no_std]

extern crate alloc;

mod adler32;
mod align;
mod bits;
mod codes;
mod copy;
mod copy_words;
mod crc32;
mod dynamic;
mod emit;
mod fast;
mod fixed;
mod gzip;
mod gzip_header;
mod gzip_member;
mod huff;
mod huff_build;
mod huff_fill;
mod huff_sub;
mod inflate_raw;
mod meta;
mod out;
mod stored;
mod tables;
mod types;
mod zlib;

pub use gzip::{gunzip, gunzip_partial};
pub use inflate_raw::{inflate, raw_partial};
pub use tables::MAX_OUT;
pub use types::{End, Inflated};
pub use zlib::{zlib, zlib_partial};
