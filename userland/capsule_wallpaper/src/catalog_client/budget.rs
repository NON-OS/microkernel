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

//! How long each catalog call may take. The catalog reads a wallpaper out of
//! the store and checks it against its pin when a chunk of one it does not
//! hold is asked for, and serves the rest from memory. Every call used to get
//! 500 ms, set when the catalog held each picture in its own binary; reading
//! and hashing one under emulation takes longer than that.
//!
//! The read is not only at offset 0. A download resumes where its last try
//! stopped (download.rs), and the catalog lets a wallpaper go once its last
//! chunk is served, even to a try that had already given up on it; the next
//! try then asks for that chunk of a wallpaper the catalog no longer holds.
//! So the first chunk of every try gets the long budget, whatever its offset.

/// The size: an answer from the catalog's own table.
pub const SIZE_MS: u64 = 500;

/// The first chunk a try asks for: a read from the store may come first.
pub const FIRST_CHUNK_MS: u64 = 15_000;

/// Every later chunk of the try: served from the bytes already held.
pub const NEXT_CHUNK_MS: u64 = 1_000;

/// The budget for a chunk, the first of its try or not.
pub fn chunk_ms(first_of_try: bool) -> u64 {
    if first_of_try {
        FIRST_CHUNK_MS
    } else {
        NEXT_CHUNK_MS
    }
}
