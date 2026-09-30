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

/*
 * An import fed by its caller, chunk by chunk, instead of read from the
 * disk plan: the kernel seals and hashes each chunk as it arrives, so the
 * file is never whole in memory, and links it under its name only when the
 * SHA-256 of everything sealed is the digest named at the start. Progress
 * is marked on the volume, so a cut stream goes on where it stopped.
 */

mod begin;
mod error;
mod finish;
mod hash;
mod hash_save;
mod link;
mod live;
mod mark;
mod mark_codec;
mod owner;
mod pause;
mod reread;
mod resume;
mod write;

pub use begin::{stream_begin, Begun};
pub use error::StreamError;
pub use finish::stream_finish;
pub use write::stream_write;
