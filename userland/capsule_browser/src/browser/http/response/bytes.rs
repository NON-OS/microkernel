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

/* Byte-level helpers for header fields, which are bytes rather than text:
a value may carry obs-text (0x80..=0xFF, RFC 9110 5.5), kept opaque. */

pub fn is_ows(c: u8) -> bool {
    c == b' ' || c == b'\t'
}

/* `b` without leading and trailing SP and HTAB. */
pub fn trim(b: &[u8]) -> &[u8] {
    let start = b.iter().position(|&c| !is_ows(c)).unwrap_or(b.len());
    let end = b.iter().rposition(|&c| !is_ows(c)).map_or(start, |e| e + 1);
    &b[start..end.max(start)]
}

/* A token character (RFC 9110 5.6.2), what a field name is made of. */
pub fn is_tchar(c: u8) -> bool {
    c.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&c)
}

/* The comma-separated elements of a list value, trimmed, empty ones
dropped (RFC 9110 5.6.1). */
pub fn list(v: &[u8]) -> impl Iterator<Item = &[u8]> {
    v.split(|&c| c == b',').map(trim).filter(|e| !e.is_empty())
}
