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

//! A pasted line into the process filter.

use nonos_app_skeleton::input::text::paste_char;

/// How a paste into the filter went.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum QueryPaste {
    /// The query now holds `len` bytes; `cut` when the line did not all fit.
    Took { len: usize, cut: bool },
    /// The line held a character no process name has, so none of it went in.
    Refused,
    /// Nothing printable to paste.
    Empty,
}

/// Append a pasted line to the query held in `buf[..len]`, as typing it
/// would: printable ASCII, the alphabet process names are written in, a tab
/// as a space and other control characters dropped. A line holding anything
/// else (a paragraph with curly quotes, say) is refused whole rather than
/// pasted with holes in it; what does not fit in `buf` is left off.
pub fn paste_query(buf: &mut [u8], len: usize, line: &str) -> QueryPaste {
    let mut chars = line.chars().filter_map(paste_char).peekable();
    if chars.peek().is_none() {
        return QueryPaste::Empty;
    }
    if !line.chars().filter_map(paste_char).all(|c| matches!(c, ' '..='~')) {
        return QueryPaste::Refused;
    }
    let mut at = len.min(buf.len());
    for ch in chars {
        if at >= buf.len() {
            return QueryPaste::Took { len: at, cut: true };
        }
        buf[at] = ch as u8;
        at += 1;
    }
    QueryPaste::Took { len: at, cut: false }
}
