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

mod bits;
mod codes;
mod crc32;
mod dynamic;
mod fixed;
mod gzip;
mod gzip_header;
mod huff;
mod inflate_raw;
mod members;
mod stored;
mod tables;
mod zlib;

pub use gzip::gunzip;
pub use inflate_raw::inflate;
pub use members::{members, Member};
pub use zlib::zlib;
