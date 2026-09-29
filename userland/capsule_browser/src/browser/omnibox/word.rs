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

/* Caret stepping over a URL: by character on UTF-8 boundaries, and by word,
 * where the separators a URL is built from end a word. */

pub fn is_sep(c: char) -> bool {
    matches!(c, '/' | '.' | ':' | '?' | '&' | '=' | '#' | '-' | '_' | ' ')
}

/* Byte offset of the character before `i`, or 0. */
pub fn prev_char(text: &str, i: usize) -> usize {
    text[..i].char_indices().next_back().map_or(0, |(j, _)| j)
}

/* Byte offset just past the character at `i`, or `i` at the end. */
pub fn next_char(text: &str, i: usize) -> usize {
    text[i..].chars().next().map_or(i, |c| i + c.len_utf8())
}

fn sep_at(text: &str, i: usize) -> bool {
    text[i..].chars().next().is_some_and(is_sep)
}

/* Start of the word before `i`: separators right before it are skipped,
 * then the word itself. */
pub fn prev_word(text: &str, i: usize) -> usize {
    let mut j = i;
    while j > 0 && sep_at(text, prev_char(text, j)) {
        j = prev_char(text, j);
    }
    while j > 0 && !sep_at(text, prev_char(text, j)) {
        j = prev_char(text, j);
    }
    j
}

/* End of the word after `i`: separators right after it are skipped, then
 * the word itself. */
pub fn next_word(text: &str, i: usize) -> usize {
    let mut j = i;
    while j < text.len() && sep_at(text, j) {
        j = next_char(text, j);
    }
    while j < text.len() && !sep_at(text, j) {
        j = next_char(text, j);
    }
    j
}
