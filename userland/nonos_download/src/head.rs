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

//! A response head, read for what a download needs of it.

use alloc::string::String;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Head {
    pub status: u16,
    pub location: Option<String>,
    pub length: Option<u64>,
    /// `Content-Range: bytes first-last/total`: first, and total if known.
    pub range: Option<(u64, Option<u64>)>,
    pub chunked: bool,
    pub content_type: Option<String>,
}

/// The head, from the bytes up to and including the blank line, or `None`
/// when they are not an HTTP/1.x response head.
pub fn parse_head(raw: &[u8]) -> Option<Head> {
    let text = core::str::from_utf8(raw).ok()?;
    let mut lines = text.split("\r\n");
    let status_line = lines.next()?;
    let mut parts = status_line.splitn(3, ' ');
    let version = parts.next()?;
    if !version.starts_with("HTTP/1.") {
        return None;
    }
    let status = parts.next()?.parse::<u16>().ok().filter(|s| (100..600).contains(s))?;
    let mut head = Head { status, ..Head::default() };
    for line in lines {
        let Some((name, value)) = line.split_once(':') else { continue };
        let value = value.trim();
        match name.trim().to_ascii_lowercase().as_str() {
            "location" => head.location = Some(String::from(value)),
            "content-length" => head.length = value.parse::<u64>().ok(),
            "content-range" => head.range = content_range(value),
            "transfer-encoding" => {
                head.chunked = value.to_ascii_lowercase().split(',').any(|c| c.trim() == "chunked")
            }
            "content-type" => head.content_type = Some(value.to_ascii_lowercase()),
            _ => {}
        }
    }
    Some(head)
}

fn content_range(value: &str) -> Option<(u64, Option<u64>)> {
    let rest = value.strip_prefix("bytes ")?;
    let (span, total) = rest.split_once('/')?;
    let first = span.split_once('-')?.0.trim().parse::<u64>().ok()?;
    Some((first, total.trim().parse::<u64>().ok()))
}
