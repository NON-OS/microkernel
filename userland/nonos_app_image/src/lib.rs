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

//! An app's prebuilt image: the files of the app and every package under it,
//! built once offline, fetched whole, installed with no package index and no
//! resolution on the device. Two objects:
//!
//! - the image (`NONOSAI1`): a header, a member table, the member names and
//!   each member's bytes, one tar per package, stored or xz-compressed;
//! - its chunk list (`NONOSAC1`): the image's length, a chunk size and the
//!   BLAKE3 of each chunk. Its own BLAKE3 is the image's measurement, the
//!   pin the market signs, so every chunk is checked against a signed digest
//!   before it is used.
//!
//! Parsing holds untrusted bytes to the format's bounds and refuses the
//! whole object on any fault. Decompression is the caller's; a member's raw
//! bytes are held to the digest the table gives them (`Member::holds`).

#![no_std]

extern crate alloc;

mod build;
mod build_list;
mod chunks;
mod format;
mod head;
mod header;
mod member;
mod name;
mod plan;
mod refused;
mod span;

pub use build::{build, Codec, Part};
pub use build_list::chunk_list;
pub use chunks::ChunkList;
pub use format::{
    CHUNK_MAX, CHUNK_MIN, HEADER_LEN, IMAGE_MAGIC, LIST_HEAD, LIST_MAGIC, MAX_IMAGE, MAX_MEMBERS,
    MAX_RAW, MEMBER_LEN, NAME_MAX, VERSION,
};
pub use head::Head;
pub use header::head_len;
pub use member::{Member, Role};
pub use plan::next;
pub use refused::Refused;
pub use span::covering;
