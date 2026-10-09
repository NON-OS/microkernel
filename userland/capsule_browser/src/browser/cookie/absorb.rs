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

//! The Set-Cookie lines of a raw response.

use alloc::string::String;
use alloc::vec::Vec;

use crate::browser::http::response::each_field;

/// How many Set-Cookie lines one response is read for.
const MAX_LINES: usize = 64;

/*
 * Set-Cookie is the one field that may not be folded into a list (RFC 9110
 * section 5.3), so each line is its own cookie. The fields are read with
 * the same reader the response framing uses, so a header the framing
 * understood is the header the jar sees.
 */
/// Each Set-Cookie value in the head of `raw`, in order. Nothing for a
/// response with no complete head.
pub fn set_cookie_lines(raw: &[u8]) -> Vec<String> {
    let mut out = Vec::new();
    let Some(head_end) = raw.windows(4).position(|w| w == b"\r\n\r\n") else {
        return out;
    };
    let Some(line_end) = raw.windows(2).position(|w| w == b"\r\n") else {
        return out;
    };
    let fields = &raw[line_end + 2..head_end + 4];
    each_field(fields, |name, value| {
        if out.len() < MAX_LINES && name.eq_ignore_ascii_case(b"set-cookie") {
            out.push(String::from_utf8_lossy(value).into_owned());
        }
    });
    out
}
