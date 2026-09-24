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

//! Splitting a response and inflating the body.

extern crate alloc;

use alloc::vec::Vec;

/// Why a directory response was not usable.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ResponseError {
    NoHeaders,
    /// A status other than 200.
    Status,
    Encoding,
}

/// The decompressed body of a `200` response.
///
pub fn body(raw: &[u8]) -> Result<Vec<u8>, ResponseError> {
    let head_end = find(raw, b"\r\n\r\n").ok_or(ResponseError::NoHeaders)?;
    let status = raw.get(..head_end).ok_or(ResponseError::NoHeaders)?;
    if !status.starts_with(b"HTTP/1.0 200") && !status.starts_with(b"HTTP/1.1 200") {
        return Err(ResponseError::Status);
    }
    let compressed = raw.get(head_end + 4..).ok_or(ResponseError::NoHeaders)?;
    if compressed.is_empty() {
        return Ok(Vec::new());
    }
    nonos_inflate::zlib(compressed).ok_or(ResponseError::Encoding)
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if haystack.len() < needle.len() {
        return None;
    }
    (0..=haystack.len() - needle.len()).find(|at| &haystack[*at..*at + needle.len()] == needle)
}
