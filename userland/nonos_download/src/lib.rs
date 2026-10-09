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
//! HTTP/1.1 for a client, with no I/O of its own.

//! What a download needs decided, kept apart from the network and the file
//! it is carried over and written to, so every rule here is held on the host.
//!
//! Music (or the Terminal) owns the stream and the file. This crate reads the
//! URL, writes the request, reads each response head into a decision
//! (follow a redirect, take the body from here, or refuse with a sentence),
//! decodes a chunked body as it comes, keeps the size limit, and says whether
//! what came is audio before anything is played.

#![no_std]

extern crate alloc;

mod audio;
mod chunked;
mod head;
mod limit;
mod plan;
mod request;
mod url;

pub use audio::{audio_start, Audio};
pub use chunked::{Chunked, ChunkError};
pub use head::{parse_head, Head};
pub use limit::{MAX_BYTES, TOO_LARGE};
pub use plan::{decide, Decision, MAX_REDIRECTS};
pub use request::request;
pub use url::{parse, resolve, DlUrl};
