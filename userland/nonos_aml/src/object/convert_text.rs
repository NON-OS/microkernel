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

//! The implicit conversion to a string (section 19.3.5.7).

use super::hex::hex_fixed;
use super::{Object, Width};
use alloc::vec::Vec;

impl Object {
    /// Integer to String is the integer in uppercase hexadecimal at the full
    /// width; Buffer to String is each byte as two hexadecimal characters,
    /// separated by spaces. Both forms are unverified against the text.
    pub fn as_text(&self, w: Width) -> Option<Vec<u8>> {
        let mut out = Vec::new();
        match self {
            Object::Integer(v) => hex_fixed(w.mask(*v), w.bytes() * 2, &mut out),
            Object::String(s) => out.extend_from_slice(&s.borrow()),
            Object::Buffer(b) => {
                for (i, x) in b.borrow().iter().enumerate() {
                    if i > 0 {
                        out.push(b' ');
                    }
                    hex_fixed(*x as u64, 2, &mut out);
                }
            }
            _ => return None,
        }
        Some(out)
    }
}
