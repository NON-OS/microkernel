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

//! Brotli decompression (RFC 7932) for no_std with alloc: the whole
//! input in, the whole output out, never past a caller's size cap.

#![no_std]

extern crate alloc;

mod bits;
mod blocks;
mod cmap;
mod complex;
mod decode;
mod dict;
mod distance;
mod error;
mod ferment;
mod frame;
mod header;
mod huff;
mod lencode;
mod lengths;
mod literal;
mod lut;
mod metablock;
mod prefix;
mod run;
mod symbol;
mod transform;
mod transforms;
mod word;

pub use decode::decompress;
pub use error::Error;
