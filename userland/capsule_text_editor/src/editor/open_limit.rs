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

//! How much of a file an open reads, and whether what came back may become the
//! document.
//!
//! The vfs client stops at the byte limit it is given and hands back what it
//! has without saying the file went on, so a read limited to exactly the
//! document's capacity cannot tell a file that fits from one that was cut off.
//! Opened, the cut-off file would look whole, and the next save would write the
//! shortened text over the original. One byte more than the capacity tells
//! them apart: a reply longer than the capacity is a file too large to open.

use super::state::CAPACITY;

/// The byte limit an open reads a file with.
pub const READ_LIMIT: u32 = CAPACITY as u32 + 1;

/// Why the bytes an open read cannot become the document, or `None` when they
/// can.
pub fn refuse_open(bytes: &[u8]) -> Option<&'static [u8]> {
    if bytes.len() > CAPACITY {
        return Some(b"open refused: file is larger than 256 KiB");
    }
    if core::str::from_utf8(bytes).is_err() {
        return Some(b"open refused: file is not valid UTF-8");
    }
    None
}
