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

//! A buffer holding a line, and one typed into, for the text proofs.

use crate::text::cache::STRING_CAP;
use crate::text::edit_buffer::EditBuffer;

pub(crate) fn held(b: &EditBuffer) -> &str {
    core::str::from_utf8(b.as_slice()).expect("the buffer is always UTF-8")
}

pub(crate) fn typed(s: &str) -> EditBuffer {
    let mut b = EditBuffer::empty();
    for ch in s.chars() {
        assert!(b.push_char(ch, STRING_CAP));
    }
    b
}
