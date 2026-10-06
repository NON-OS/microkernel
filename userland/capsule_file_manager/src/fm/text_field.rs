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

//! What the one-line fields take: the name prompt, the filter and the search
//! box. They hold UTF-8 in a `String`, so Backspace (`String::pop`) always
//! takes a whole character; these say which characters go in and how many
//! bytes each field holds.

use alloc::string::String;

/// The longest name the prompt takes, in bytes.
pub const NAME_MAX: usize = 64;
/// The longest filter, in bytes.
pub const FILTER_MAX: usize = 48;
/// The longest search query, in bytes.
pub const QUERY_MAX: usize = 64;

/// A character a file name or filter may hold: any printable one but white
/// space, as before, only no longer limited to ASCII, so a file can be named
/// in the language its owner writes. Control characters never reach here.
pub fn name_char(ch: char) -> bool {
    !ch.is_whitespace() && !ch.is_control()
}

/// A character the search query may hold: a name's, or a space between words.
pub fn query_char(ch: char) -> bool {
    ch == ' ' || name_char(ch)
}

/// How a paste into a field went.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FieldPaste {
    /// All of it went in.
    Whole,
    /// It went in up to what the field holds.
    Cut,
    /// It held a character the field refuses, so none of it went in.
    Refused,
    /// There was nothing to paste.
    Empty,
}

/// Append a pasted line to `field` as typing it would. A tab reads as a
/// space and other control characters are dropped (app_skeleton's
/// `paste_char`, the same rule every app's paste follows); a line with any
/// character `allow` refuses is refused whole, since a name pasted with the
/// refused characters silently gone names a different file. What does not
/// fit in `max` bytes is left off, whole characters only.
pub fn paste_within(
    field: &mut String,
    line: &str,
    max: usize,
    allow: fn(char) -> bool,
) -> FieldPaste {
    let pasted = || {
        line.chars().filter_map(|c| match c {
            '\t' => Some(' '),
            c if c.is_control() => None,
            c => Some(c),
        })
    };
    if pasted().next().is_none() {
        return FieldPaste::Empty;
    }
    if !pasted().all(allow) {
        return FieldPaste::Refused;
    }
    for ch in pasted() {
        if !push_within(field, ch, max, allow) {
            return FieldPaste::Cut;
        }
    }
    FieldPaste::Whole
}

/// Append `ch` to `field` if `allow` takes it and it fits whole in `max`
/// bytes. Returns whether it went in.
pub fn push_within(field: &mut String, ch: char, max: usize, allow: fn(char) -> bool) -> bool {
    if !allow(ch) || field.len() + ch.len_utf8() > max {
        return false;
    }
    field.push(ch);
    true
}
