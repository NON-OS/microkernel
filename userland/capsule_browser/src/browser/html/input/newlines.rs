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

use alloc::string::String;

/// Append `s` with every CR LF pair and every lone CR written as one LF, so
/// nothing after this sees a carriage return the page did not escape.
pub fn push_normalized(out: &mut String, s: &str) {
    let b = s.as_bytes();
    let mut from = 0;
    while let Some(off) = b[from..].iter().position(|&c| c == b'\r') {
        let at = from + off;
        out.push_str(&s[from..at]);
        out.push('\n');
        from = if b.get(at + 1) == Some(&b'\n') { at + 2 } else { at + 1 };
    }
    out.push_str(&s[from..]);
}
