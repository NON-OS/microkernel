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

//! Finding URLs in text a program printed, so the host can offer to open
//! one. Only schemes a person would follow are recognised.

const SCHEMES: [&str; 6] = ["https://", "http://", "file://", "ftp://", "ssh://", "git://"];

fn url_char(c: char) -> bool {
    !c.is_whitespace() && !c.is_control() && !"<>\"'`{}|\\^".contains(c)
}

/// Every URL in `text`, as char index ranges `[start, end)`. Trailing
/// punctuation is dropped unless it closes a bracket the URL opened.
pub fn find_urls(text: &[char]) -> alloc::vec::Vec<(usize, usize)> {
    let mut out = alloc::vec::Vec::new();
    let mut i = 0;
    while i < text.len() {
        let hit = SCHEMES.iter().find(|s| {
            let n = s.chars().count();
            i + n <= text.len()
                && s.chars().zip(&text[i..i + n]).all(|(a, &b)| a.eq_ignore_ascii_case(&b))
        });
        let Some(s) = hit else {
            i += 1;
            continue;
        };
        let start = i;
        let mut end = i + s.len();
        while end < text.len() && url_char(text[end]) {
            end += 1;
        }
        while end > start + s.len() {
            let c = text[end - 1];
            let opens = |o: char| text[start..end].iter().filter(|&&x| x == o).count();
            let closes = |cl: char| text[start..end].iter().filter(|&&x| x == cl).count();
            let unbalanced = match c {
                ')' => closes(')') > opens('('),
                ']' => closes(']') > opens('['),
                '.' | ',' | ';' | ':' | '!' | '?' => true,
                _ => false,
            };
            if unbalanced {
                end -= 1;
            } else {
                break;
            }
        }
        if end > start + s.len() {
            out.push((start, end));
        }
        i = end.max(i + 1);
    }
    out
}
