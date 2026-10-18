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

//! The redirect or pipe operator a word starts with, and the word it stands for.

/*
 * The operator starting at `i`, as the word it stands for and where it
 * ends. `|`, `<`, `>` and `>>` stand for themselves. A run of digits right
 * before `<` or `>` names the stream: `1>`, `1>>` and `0<` are the plain
 * forms and read as `>`, `>>` and `<`; any other number, and a `>&` that
 * joins two streams, is kept whole as one word for the redirect plan to
 * refuse, so `2>f` never hands a program a stray "2".
 */
pub(super) fn operator(input: &[u8], i: usize) -> Option<(&[u8], usize)> {
    let mut j = i;
    while j < input.len() && input[j].is_ascii_digit() {
        j += 1;
    }
    let op = *input.get(j)?;
    if !matches!(op, b'<' | b'>' | b'|') || (j > i && op == b'|') {
        return None;
    }
    let mut end = j + 1;
    if op == b'>' && input.get(end) == Some(&b'>') {
        end += 1;
    }
    let joins = op != b'|' && input.get(end) == Some(&b'&');
    if joins {
        end += 1;
        while end < input.len() && (input[end].is_ascii_digit() || input[end] == b'-') {
            end += 1;
        }
    }
    let plain = match &input[i..j] {
        b"" => true,
        b"1" => op == b'>',
        b"0" => op == b'<',
        _ => false,
    };
    if plain && !joins {
        return Some((&input[j..end], end));
    }
    Some((&input[i..end], end))
}
