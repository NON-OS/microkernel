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

/* What a parse was made from: the CSS text's length and FNV-1a hash (the
 * pair makes a same-length edit a miss), and whether the text ends
 * outside every block, string and comment, so text appended to it parses
 * the same alone as it would after it. */
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct TextKey {
    pub len: usize,
    pub hash: u64,
    pub closed: bool,
}

pub(super) const FNV_START: u64 = 0xcbf2_9ce4_8422_2325;

pub(super) fn fnv(mut h: u64, bytes: &[u8]) -> u64 {
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/* Whether `css` ends at brace depth 0, never below it, outside a string
 * and a comment, and after a whole rule or statement. */
pub(super) fn closed(css: &str) -> bool {
    let b = css.as_bytes();
    let (mut depth, mut quote, mut comment, mut i) = (0i64, 0u8, false, 0usize);
    let mut last = b'}';
    while i < b.len() {
        let c = b[i];
        if !(comment || c.is_ascii_whitespace() || (c == b'/' && b.get(i + 1) == Some(&b'*'))) {
            last = c;
        }
        if comment {
            if c == b'*' && b.get(i + 1) == Some(&b'/') {
                (comment, i) = (false, i + 1);
            }
        } else if c == b'\\' {
            i += 1;
        } else if quote != 0 {
            if c == quote || c == b'\n' {
                quote = 0;
            }
        } else {
            match c {
                b'"' | b'\'' => quote = c,
                b'/' if b.get(i + 1) == Some(&b'*') => (comment, i) = (true, i + 1),
                b'{' => depth += 1,
                b'}' if depth == 0 => return false,
                b'}' => depth -= 1,
                _ => {}
            }
        }
        i += 1;
    }
    depth == 0 && quote == 0 && !comment && i == b.len() && matches!(last, b'}' | b';')
}
