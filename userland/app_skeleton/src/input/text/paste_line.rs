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

//! What a one-line field takes from the clipboard.

/// The first line of the clipboard's text.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PasteLine<'a> {
    pub text: &'a str,
    /// More lines followed it, which a one-line field leaves behind.
    pub more: bool,
}

/// The first line of `bytes`, the clipboard's text as read into a buffer.
///
/// None when the bytes are not text: invalid UTF-8 anywhere in the line is
/// refused whole rather than pasted as garbage. A character cut off at the
/// end of the buffer (the clipboard held more than it) is dropped, and what
/// came before it is kept. The line ends at the first newline or carriage
/// return; leading and trailing white space is trimmed, since a copied word
/// so often brings the space or newline beside it.
pub fn first_line(bytes: &[u8]) -> Option<PasteLine<'_>> {
    // Whether junk followed the first line: more, though not text.
    let mut junk = false;
    let text = match core::str::from_utf8(bytes) {
        Ok(t) => t,
        // error_len None: the bytes end inside a character, a cut, not junk.
        Err(e) if e.error_len().is_none() => {
            core::str::from_utf8(&bytes[..e.valid_up_to()]).ok()?
        }
        Err(e) => {
            // Junk past the first line is never pasted, so the line still
            // is; the junk only counts as more.
            let head = &bytes[..e.valid_up_to()];
            if !head.iter().any(|&b| b == b'\n' || b == b'\r') {
                return None;
            }
            junk = true;
            core::str::from_utf8(head).ok()?
        }
    };
    let text = text.trim_start_matches(['\r', '\n']);
    let (line, more) = match text.find(['\n', '\r']) {
        Some(cut) => (&text[..cut], !text[cut..].trim().is_empty()),
        None => (text, false),
    };
    Some(PasteLine { text: line.trim(), more: more || junk })
}

/// What a pasted character becomes in a one-line field: a tab a space, and
/// any other control character nothing, since none of them can be typed.
pub fn paste_char(ch: char) -> Option<char> {
    match ch {
        '\t' => Some(' '),
        c if c.is_control() => None,
        c => Some(c),
    }
}
