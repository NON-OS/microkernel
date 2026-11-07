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

//! The broker's wire: a 16-byte header, then the op's fields.
//!
//! ```text
//! request: magic "NOUF" u32, version u16, op u16, request id u32,
//!          body length u32, body
//! reply:   the same header with the request's op and id, body length
//!          4 + n, then status i32 (0, or a negated errno), then n bytes
//! ```
//!
//! Every field is little-endian. A path is the rest of the body after the
//! op's fixed fields, so it has no length to lie about.

pub mod client;
mod client_names;
mod client_read;
pub mod flags;
mod header;
pub mod op;
mod reply;
mod request;

pub use header::{Header, HEADER};
pub use reply::{attr_bytes, kind_byte, Reply, ATTR_LEN};
pub use request::Request;

/// "NOUF".
pub const MAGIC: u32 = u32::from_le_bytes(*b"NOUF");
pub const VERSION: u16 = 1;
/// A request or reply is at most this long: the largest write and its
/// header with room to spare, under the kernel's 1 MiB message.
pub const MESSAGE_MAX: usize = crate::limits::IO_MAX + 4096;
