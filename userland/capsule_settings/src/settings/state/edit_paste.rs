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

//! A pasted line into one of the panel's fields.

use nonos_app_skeleton::input::text::paste_char;

use super::edit_buffer::EditBuffer;

/// How a paste into a field went.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pasted {
    /// All of it went in.
    Whole,
    /// It went in up to what the field holds.
    Cut,
    /// It held a character the field refuses, so none of it went in.
    Refused,
    /// There was nothing to paste.
    Empty,
}

/// What a stored string value may hold: the policy store refuses any other
/// character (capsule_policy store/str_validate.rs), so the value editor
/// takes none, typed or pasted.
pub fn value_char(ch: char) -> bool {
    matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '.' | '_')
}

impl EditBuffer {
    /// Append a pasted line as typing it would, one character at a time: a
    /// tab as a space, other control characters dropped. `allow` is the
    /// field's own rule, and a line with any character it refuses is refused
    /// whole: a hostname pasted with a space in it, or a passphrase with a
    /// letter WPA cannot carry, is a different value with the bad character
    /// silently gone. The field holds at most `cap` bytes and `max_chars`
    /// characters; what does not fit is left off, whole characters only.
    pub fn paste(
        &mut self,
        line: &str,
        cap: usize,
        max_chars: usize,
        allow: impl Fn(char) -> bool,
    ) -> Pasted {
        let mut chars = line.chars().filter_map(paste_char).peekable();
        if chars.peek().is_none() {
            return Pasted::Empty;
        }
        if !line.chars().filter_map(paste_char).all(&allow) {
            return Pasted::Refused;
        }
        for ch in chars {
            if self.char_count() >= max_chars || !self.push_char(ch, cap) {
                return Pasted::Cut;
            }
        }
        Pasted::Whole
    }
}
