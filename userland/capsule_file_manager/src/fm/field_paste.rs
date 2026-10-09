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

//! Ctrl+V and Shift+Insert in the name prompt, the filter and the search
//! box: the clipboard's text (the system clipboard, not the files copied
//! with c and x) into the field, by the field's own rule.

use alloc::string::String;

use nonos_app_skeleton::clipboard_paste_line;

use super::text_field::{paste_within, FieldPaste};

/// Paste into `field`. Returns what the footer should say, or None when the
/// whole line went in and there is nothing to say; `refused` is what it says
/// when the line holds a character the field does not take.
pub fn paste_field(
    field: &mut String,
    max: usize,
    allow: fn(char) -> bool,
    refused: &'static [u8],
) -> Option<&'static [u8]> {
    // Room for more than any field holds, so a longer line is seen as cut.
    let mut buf = [0u8; 256];
    let line = match clipboard_paste_line(&mut buf) {
        Ok(Some(line)) => line,
        Ok(None) => return Some(b"paste: the clipboard holds no text"),
        Err(_) => return Some(b"paste: the clipboard is not available"),
    };
    match paste_within(field, line.text, max, allow) {
        FieldPaste::Whole => None,
        FieldPaste::Cut => Some(b"paste: cut to what the field holds"),
        FieldPaste::Refused => Some(refused),
        FieldPaste::Empty => Some(b"paste: the clipboard holds no text"),
    }
}
