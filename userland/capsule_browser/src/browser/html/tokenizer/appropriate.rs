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

use super::state::{is_ws, Tokenizer};

impl<'a> Tokenizer<'a> {
    /// Whether an end tag for the last start tag begins at `at`: "</", the
    /// same letters in any case, then whitespace, "/" or ">".
    pub(super) fn appropriate_end(&self, at: usize) -> bool {
        let b = self.src.as_bytes();
        let name = self.last_start.as_bytes();
        let start = at + 2;
        let end = start + name.len();
        !name.is_empty()
            && b.get(at..start) == Some(b"</".as_slice())
            && end < b.len()
            && b[start..end].iter().all(u8::is_ascii_alphabetic)
            && b[start..end].eq_ignore_ascii_case(name)
            && (is_ws(b[end]) || b[end] == b'/' || b[end] == b'>')
    }
}
