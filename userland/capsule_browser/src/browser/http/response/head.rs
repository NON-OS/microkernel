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

use super::block::{block_end, skip_blanks, Block};
use super::head_fields::read_fields;
use super::status_code::status_code;

pub use super::head_types::{Framing, Head, Scan};

/* The one reading of a response head that parse, completion, keep-alive
framing and the stash share. Interim 1xx responses other than 101 are
skipped (RFC 9110 15.2); the first other response is the final one. */
pub fn scan(raw: &[u8]) -> Scan {
    let mut from = 0;
    loop {
        from = skip_blanks(raw, from);
        let end = match block_end(raw, from) {
            Block::End(end) => end,
            Block::More => return Scan::More,
            Block::Bad => return Scan::Bad,
        };
        let Some((line, next)) = line_end(&raw[from..]) else { return Scan::Bad };
        let Some((status, http10)) = status_code(&raw[from..from + line]) else { return Scan::Bad };
        if (100..200).contains(&status) && status != 101 {
            from = end;
            continue;
        }
        return read_fields(&raw[from + next..end], status, http10, end);
    }
}
