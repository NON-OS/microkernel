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

use crate::browser::http::chunked::line_end;

/* A header section larger than this is refused, as Chromium does. */
const MAX_HEAD: usize = 256 * 1024;

/* Stray empty lines skipped before a status line, which some servers
leave after a body on a kept-alive connection. */
const MAX_LEADING_BLANKS: usize = 4;

pub enum Block {
    /* The section's blank line ends just before this offset. */
    End(usize),
    /* It has not all arrived. */
    More,
    /* It is not an HTTP/1 header section. */
    Bad,
}

/* Where the message at `from` really starts, after stray empty lines. */
pub fn skip_blanks(raw: &[u8], mut from: usize) -> usize {
    for _ in 0..MAX_LEADING_BLANKS {
        match line_end(&raw[from..]) {
            Some((0, next)) => from += next,
            _ => break,
        }
    }
    from
}

/* The header section of the message starting at `from`: status line,
field lines, blank line. A line ends at LF with or without CR. */
pub fn block_end(raw: &[u8], from: usize) -> Block {
    let rest = &raw[from..];
    let n = rest.len().min(5);
    if rest[..n] != b"HTTP/"[..n] {
        return Block::Bad;
    }
    let mut i = 0;
    loop {
        let Some((end, next)) = line_end(&rest[i..]) else {
            return if rest.len() > MAX_HEAD { Block::Bad } else { Block::More };
        };
        i += next;
        if end == 0 {
            return Block::End(from + i);
        }
        if i > MAX_HEAD {
            return Block::Bad;
        }
    }
}
