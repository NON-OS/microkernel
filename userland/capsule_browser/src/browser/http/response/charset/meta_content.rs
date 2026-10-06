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

use super::encoding::Encoding;
use super::label::{encoding, is_ascii_space};

/* HTML's "extracting a character encoding from a meta element": the first
`charset` followed by '=', its value quoted (with a closing quote) or
bare up to whitespace or ';'. `s` is already lowercase. */
pub fn from_content(s: &[u8]) -> Option<Encoding> {
    let mut pos = 0;
    loop {
        pos += s.get(pos..)?.windows(7).position(|w| w == b"charset")? + 7;
        while s.get(pos).is_some_and(|&c| is_ascii_space(c)) {
            pos += 1;
        }
        if s.get(pos) != Some(&b'=') {
            continue;
        }
        pos += 1;
        while s.get(pos).is_some_and(|&c| is_ascii_space(c)) {
            pos += 1;
        }
        let &c = s.get(pos)?;
        if c == b'"' || c == b'\'' {
            let len = s[pos + 1..].iter().position(|&d| d == c)?;
            return encoding(&s[pos + 1..pos + 1 + len]);
        }
        let len =
            s[pos..].iter().position(|&d| is_ascii_space(d) || d == b';').unwrap_or(s.len() - pos);
        return encoding(&s[pos..pos + len]);
    }
}
