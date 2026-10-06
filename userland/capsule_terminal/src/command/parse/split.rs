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

use super::types::{Argv, MAX_ARGS};
use crate::term::util::is_space;

/// Outside quotes these end a word and stand as words of their own, as a
/// POSIX shell reads them: `a>b`, `<f` and `ls|grep x` hold operators.
const fn ends_word(byte: u8) -> bool {
    is_space(byte) || matches!(byte, b'"' | b'\'' | b'<' | b'>' | b'|')
}

pub fn parse(input: &[u8]) -> Argv<'_> {
    let mut out = Argv { argv: [b""; MAX_ARGS], argc: 0 };
    let mut i = 0;
    while i < input.len() && out.argc < MAX_ARGS {
        while i < input.len() && is_space(input[i]) {
            i += 1;
        }
        if i >= input.len() {
            break;
        }
        // A token wrapped in matching single or double quotes is taken
        // verbatim between the quotes, so arguments may contain spaces
        // (`write notes.txt "hello world"`) and `<`, `>` and `|` stay text
        // (`echo "a>b"`). The slice still points into the input; only the
        // quote bytes are excluded. An unterminated quote runs to end of line.
        if input[i] == b'"' || input[i] == b'\'' {
            let quote = input[i];
            let start = i + 1;
            let mut j = start;
            while j < input.len() && input[j] != quote {
                j += 1;
            }
            out.argv[out.argc] = &input[start..j];
            i = if j < input.len() { j + 1 } else { j };
        } else if let Some((word, end)) = operator(input, i) {
            out.argv[out.argc] = word;
            i = end;
        } else {
            let start = i;
            while i < input.len() && !ends_word(input[i]) {
                i += 1;
            }
            out.argv[out.argc] = &input[start..i];
        }
        out.argc += 1;
    }
    out
}

/*
 * The operator starting at `i`, as the word it stands for and where it
 * ends. `|`, `<`, `>` and `>>` stand for themselves. A run of digits right
 * before `<` or `>` names the stream: `1>`, `1>>` and `0<` are the plain
 * forms and read as `>`, `>>` and `<`; any other number, and a `>&` that
 * joins two streams, is kept whole as one word for the redirect plan to
 * refuse, so `2>f` never hands a program a stray "2".
 */
fn operator(input: &[u8], i: usize) -> Option<(&[u8], usize)> {
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
