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

//! The person's files: Documents, Downloads, Music, Pictures and Videos,
//! kept on the data volume as
//! numbered objects, served to clients over IPC.
//!
//! A file is a list of 1 MiB chunks, each an object written whole and never
//! written over; the tree is a manifest committed into one of two slot
//! objects, so a cut at any point leaves the last committed tree whole.
//! Nothing here reaches the kernel: the capsule hands in the four object
//! calls (`Objects`) and the facts about its clients (`Host`).

#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod broker;
mod bytes;
mod chunks;
pub mod errno;
mod handles;
mod host;
pub mod limits;
pub mod manifest;
mod node;
pub mod path;
mod serve;
mod store;
mod tree;
pub mod wire;

pub use broker::Broker;
pub use host::{Host, Objects};
pub use node::{Attr, Kind};
pub use serve::serve;
pub use tree::{DOCUMENTS, DOWNLOADS, FOLDERS, MUSIC, PICTURES, ROOT, VIDEOS};
