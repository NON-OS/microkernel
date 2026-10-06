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

use super::c1::c1;

/// A numeric reference, `rest` starting at its "#". Appends the character
/// and returns the bytes used, the closing semicolon included when present.
///
/// Digits saturate rather than wrap, so a long run of them is out of range
/// instead of landing on some unrelated character. Zero, surrogates and
/// numbers past Unicode become U+FFFD; other controls and noncharacters are
/// kept, as the specification says. With no digits at all, "&#" or "&#x"
/// is ordinary text.
pub fn numeric_ref(rest: &str, out: &mut String) -> usize {
    let b = rest.as_bytes();
    let hex = matches!(b.get(1), Some(b'x' | b'X'));
    let (start, radix) = if hex { (2, 16) } else { (1, 10) };
    let mut end = start;
    let mut code: u32 = 0;
    while let Some(d) = b.get(end).and_then(|&c| (c as char).to_digit(radix)) {
        code = code.saturating_mul(radix).saturating_add(d).min(0x11_0000);
        end += 1;
    }
    if end == start {
        out.push('&');
        out.push_str(&rest[..start]);
        return start;
    }
    if b.get(end) == Some(&b';') {
        end += 1;
    }
    out.push(resolve(code));
    end
}

/// A surrogate or a number past U+10FFFF is not a character at all, which is
/// exactly when `from_u32` refuses; the specification replaces those, and
/// zero, with U+FFFD.
fn resolve(code: u32) -> char {
    if code == 0 {
        return '\u{FFFD}';
    }
    char::from_u32(c1(code)).unwrap_or('\u{FFFD}')
}
