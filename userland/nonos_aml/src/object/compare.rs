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

//! The ordering LEqual, LGreater and LLess use (section 19.6.70 and on): the
//! second operand is converted to the first's type; strings and buffers
//! compare byte by byte, and a proper prefix is the lesser.

use super::{Object, Width};
use core::cmp::Ordering;

pub fn compare(a: &Object, b: &Object, w: Width) -> Option<Ordering> {
    match a {
        Object::Integer(x) => Some(w.mask(*x).cmp(&b.as_integer(w)?)),
        Object::String(s) => Some(s.borrow().as_slice().cmp(b.as_text(w)?.as_slice())),
        Object::Buffer(x) => Some(x.borrow().as_slice().cmp(b.as_bytes(w)?.as_slice())),
        _ => None,
    }
}
