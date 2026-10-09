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

/// A run of character data read from the input. It stays a borrowed slice
/// until something in it is decoded (a character reference, a NUL), and only
/// then is copied, so most text reaches the tree with one copy, the one into
/// its node.
pub struct Run {
    start: usize,
    copied: usize,
    out: Option<String>,
}

impl Run {
    pub fn new(start: usize) -> Self {
        Run { start, copied: start, out: None }
    }

    /// Replace the input from `at` with what `decode` appends; `decode`
    /// returns how many input bytes that replaced, which is returned too.
    pub fn decode(
        &mut self,
        src: &str,
        at: usize,
        decode: impl FnOnce(&mut String) -> usize,
    ) -> usize {
        let out = self.out.get_or_insert_with(String::new);
        out.push_str(&src[self.copied..at]);
        let used = decode(out);
        self.copied = at + used;
        used
    }

    /// The run as text, ending at `end`.
    pub fn finish(self, src: &str, end: usize) -> Cow<'_, str> {
        match self.out {
            None => Cow::Borrowed(&src[self.start..end]),
            Some(mut out) => {
                out.push_str(&src[self.copied..end]);
                Cow::Owned(out)
            }
        }
    }
}

/// Text in which a NUL is a parse error that becomes U+FFFD, borrowed when
/// it holds none.
pub fn replace_nul(s: &str) -> Cow<'_, str> {
    if !s.contains('\0') {
        return Cow::Borrowed(s);
    }
    Cow::Owned(s.replace('\0', "\u{FFFD}"))
}

/// Whether the byte after "&" can start a character reference at all.
pub fn may_start_ref(next: Option<&u8>) -> bool {
    next.is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'#')
}
