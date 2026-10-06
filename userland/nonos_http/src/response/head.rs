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
//! One response head: the status line and the fields up to the blank line.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use super::super::error::HttpError;
use super::headers::headers;
use super::status::status;

/// The status code, the fields, and the bytes after the blank line.
pub(super) type Head<'a> = (u16, Vec<(String, String)>, &'a [u8]);

/// The most a head may take, status line and fields together. The field
/// count is capped separately; this caps their size, which a caller's
/// download limit alone left as large as the limit.
const MAX_HEAD: usize = 64 * 1024;

pub(super) fn head(raw: &[u8]) -> Result<Head<'_>, HttpError> {
    let window = &raw[..raw.len().min(MAX_HEAD + 4)];
    let split = match window.windows(4).position(|w| w == b"\r\n\r\n") {
        Some(at) => at,
        // No blank line within the cap: too large, not merely unfinished.
        None if raw.len() > MAX_HEAD + 3 => return Err(HttpError::Header),
        None => return Err(HttpError::Incomplete),
    };
    let head = &raw[..split];
    let after = &raw[split + 4..];
    let mut lines = head.split(|b| *b == b'\n');
    let first = lines.next().ok_or(HttpError::StatusLine)?;
    let code = status(first.strip_suffix(b"\r").unwrap_or(first))?;
    let rest_at = first.len() + 1;
    let fields = headers(head.get(rest_at..).unwrap_or(&[]))?;
    Ok((code, fields, after))
}
