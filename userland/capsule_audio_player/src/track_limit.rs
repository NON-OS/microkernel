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

//! The largest track the player reads whole, and the check that holds a file
//! to it.
//!
//! The vfs client stops at the byte limit it is given and hands back what it
//! has without saying the file went on, so a track read at exactly the limit
//! came back cut short and played as if it ended there. The read asks for one
//! byte more: a reply that long is a file past the limit.

/// The largest track read whole, which the heap is sized to hold.
pub const MAX_FILE: u32 = 32 * 1024 * 1024;

/// The byte limit a track is read with.
pub const READ_LIMIT: u32 = MAX_FILE + 1;

/// The load error for a file past `MAX_FILE`.
pub const TOO_LARGE: &str = "track is larger than 32 MiB";

/// Why a file of `len` bytes is not read or played, or `None` when it fits.
pub fn refuse_size(len: u64) -> Option<&'static str> {
    if len > MAX_FILE as u64 {
        Some(TOO_LARGE)
    } else {
        None
    }
}
