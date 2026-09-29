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

use super::bytes::is_ows;

/* The status line (RFC 9112 4): HTTP/1.0 or HTTP/1.1, whitespace, a
three-digit code from 100 to 599, then an optional reason phrase.
Returns the code and whether the version is HTTP/1.0. */
pub fn status_code(line: &[u8]) -> Option<(u16, bool)> {
    let http10 = match line.get(..8)? {
        b"HTTP/1.0" => true,
        b"HTTP/1.1" => false,
        _ => return None,
    };
    let rest = &line[8..];
    let skip = rest.iter().take_while(|&&c| is_ows(c)).count();
    if skip == 0 {
        return None;
    }
    let rest = &rest[skip..];
    let digits = rest.get(..3)?;
    if !digits.iter().all(u8::is_ascii_digit) || rest.get(3).is_some_and(|&c| !is_ows(c)) {
        return None;
    }
    let code = digits.iter().fold(0u16, |v, &d| v * 10 + u16::from(d - b'0'));
    (100..=599).contains(&code).then_some((code, http10))
}
