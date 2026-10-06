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

use alloc::borrow::Cow;
use alloc::string::String;

use super::state::{is_ws, Tokenizer};

impl<'a> Tokenizer<'a> {
    /// A tag name, lowercased; borrowed from the input when already so.
    pub(super) fn tag_name(&mut self) -> Cow<'a, str> {
        let src: &'a str = self.src;
        let b = src.as_bytes();
        let from = self.pos;
        let stop = b[from..]
            .iter()
            .position(|&c| is_ws(c) || c == b'/' || c == b'>')
            .map_or(b.len(), |n| from + n);
        self.pos = stop;
        let raw = &src[from..stop];
        if !raw.bytes().any(|c| c.is_ascii_uppercase() || c == 0) {
            return Cow::Borrowed(raw);
        }
        let mut name = String::with_capacity(raw.len());
        push_lower(&mut name, raw);
        Cow::Owned(name)
    }
}

/// Append a tag, attribute or doctype name: ASCII letters lowercased and a
/// NUL replaced, as the name states do. Other characters are kept as they
/// are, so a name in another script survives.
pub fn push_lower(out: &mut String, s: &str) {
    if !s.bytes().any(|c| c.is_ascii_uppercase() || c == 0) {
        out.push_str(s);
        return;
    }
    for c in s.chars() {
        match c {
            '\0' => out.push('\u{FFFD}'),
            c => out.push(c.to_ascii_lowercase()),
        }
    }
}

/// Append text in which a NUL is a parse error that becomes U+FFFD.
pub fn push_text(out: &mut String, s: &str) {
    let mut from = 0;
    while let Some(n) = s[from..].find('\0') {
        out.push_str(&s[from..from + n]);
        out.push('\u{FFFD}');
        from += n + 1;
    }
    out.push_str(&s[from..]);
}
