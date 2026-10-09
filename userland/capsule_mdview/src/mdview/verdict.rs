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

//! Whether the bytes read from /readme.txt become the page, and what the
//! window says when they do not.
//!
//! The vfs client stops at the byte limit it is given without saying the file
//! went on, so a read limited to exactly the page's budget cannot tell a file
//! that fits from one it cut short; one byte past the budget can. And a file
//! that parses to nothing is a page with nothing on it, not a read to retry.

/// The most of /readme.txt the viewer lays out.
pub const MAX_BYTES: u32 = 64 * 1024;

/// The byte limit the file is read with.
pub const READ_LIMIT: u32 = MAX_BYTES + 1;

pub const TOO_LARGE: &str = "mdview: /readme.txt is larger than 64 KiB";
pub const EMPTY: &str = "mdview: /readme.txt is empty";

/// Why `len` bytes read are not laid out, or `None` when they fit.
pub fn refuse_bytes(len: usize) -> Option<&'static str> {
    if len > MAX_BYTES as usize {
        Some(TOO_LARGE)
    } else {
        None
    }
}

/// Why a document read whole shows nothing, or `None` when it has blocks.
pub fn empty_page(blocks: usize) -> Option<&'static str> {
    if blocks == 0 {
        Some(EMPTY)
    } else {
        None
    }
}
